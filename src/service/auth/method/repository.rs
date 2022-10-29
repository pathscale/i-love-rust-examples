use std::str::FromStr;
use thiserror::Error;

use lib::database::LocalDbClientError;
use lib::id_gen::{ConcurrentSnowflake, ConcurrentSnowflakeError};
use uuid::Uuid;

use gen::database::*;
use lib::database::LocalDbClient;

use localdb::parsetools::*;

use gen::model::{EnumRecoveryQuestionCategory, EnumRole, EnumService};

pub async fn fun_auth_signup(
    db: &LocalDbClient,
    mut snowflake: ConcurrentSnowflake,
    req: FunAuthSignupReq,
) -> Result<FunAuthSignupResp, RepositoryError> {
    // creating user
    let timestamp = unix_timestamp();
    let pkey_id = snowflake.gen()?;
    db.query(
        "\
				INSERT INTO user (\
								pkey_id, \
								public_id, \
								username, \
								email, \
								phone_number, \
								password_hash, \
								password_salt, \
								age, \
								preferred_language, \
								agreed_tos, \
								agreed_privacy, \
								created_at, \
								updated_at, \
								last_ip) \
								VALUES (\
										?0, \
										?1, \
										?2, \
										?3, \
										?4, \
										?5, \
										?6, \
										?7, \
										?8, \
										?9, \
										?10, \
										?11, \
										?12, \
										?13);\
				",
        &[
            &pkey_id,
            &req.public_id,
            &req.username,
            &req.email,
            &req.phone,
            &req.password_hash,
            &req.password_salt,
            &req.age,
            &req.preferred_language,
            &req.agreed_tos,
            &req.agreed_privacy,
            &timestamp.clone(),
            &timestamp.clone(),
            &req.ip_address,
        ],
    )
    .await?;

    Ok(FunAuthSignupResp {
        rows: vec![FunAuthSignupRespRow { user_id: pkey_id }],
    })
}

pub async fn fun_auth_get_password_salt(
    db: &LocalDbClient,
    req: FunAuthGetPasswordSaltReq,
) -> Result<FunAuthGetPasswordSaltResp, RepositoryError> {
    // getting salt
    let salt = db
        .query(
            "\
						SELECT password_salt \
								FROM user \
								WHERE username = ?0;\
						",
            &[&req.username],
        )
        .await?
        .try_next_select()?
        .maybe_first_row()
        .ok_or_else(|| RepositoryError::UnknownUserGetSaltError)?
        .try_first_value()?
        .try_bytea()?;

    Ok(FunAuthGetPasswordSaltResp {
        rows: vec![FunAuthGetPasswordSaltRespRow { salt: salt }],
    })
}

pub async fn fun_auth_authenticate(
    db: &LocalDbClient,
    mut snowflake: ConcurrentSnowflake,
    req: FunAuthAuthenticateReq,
) -> Result<FunAuthAuthenticateResp, RepositoryError> {
    // looking up user
    let user_auth_row = db
        .query(
            "\
						SELECT pkey_id, public_id, password_hash, user_role, is_blocked \
								FROM user \
								WHERE username = ?0;\
						",
            &[&req.username],
        )
        .await?
        .try_next_select()?
        .maybe_first_row()
        .ok_or_else(|| RepositoryError::UnknownUserLoginError)?;

    let pkey_id = user_auth_row[0].try_i64()?;
    let public_id = user_auth_row[1].try_i64()?;
    let hash = user_auth_row[2].try_bytea()?;
    let role: EnumRole = user_auth_row[3]
        .try_string()?
        .parse()
        .map_err(|_| RepositoryError::ParseEnumRoleError)?;
    let blocked = user_auth_row[4].try_bool()?;

    // checking password ok and registering login attempt
    let password_ok = req.password_hash == hash;
    db.query(
        "\
				INSERT INTO login_attempt (\
					pkey_id, \
					fkey_user, \
					username, \
					password_hash, \
					ip_address, \
					is_password_ok, \
					moment) \
					VALUES (\
							?0, \
							?1, \
							?2, \
							?3, \
							?4, \
							?5, \
							?6);\
				",
        &[
            &snowflake.gen()?,
            &pkey_id,
            &req.username,
            &req.password_hash,
            &req.ip_address,
            &password_ok,
            &unix_timestamp(),
        ],
    )
    .await?;
    if !password_ok {
        return Err(RepositoryError::InvalidPasswordError);
    }

    // checking block status
    if blocked {
        return Err(RepositoryError::BlockedUserLoginError);
    }

    // checking authorization
    let service = EnumService::try_from(req.service_code)
        .map_err(|()| RepositoryError::ParseEnumServiceError)?;

    match service {
        EnumService::Admin => match role {
            EnumRole::Developer => (),
            EnumRole::Admin => (),
            _ => return Err(RepositoryError::InvalidRoleForAdminError),
        },
        EnumService::User => match role {
            EnumRole::Developer => (),
            EnumRole::Admin => (),
            EnumRole::User => (),
            _ => return Err(RepositoryError::InvalidRoleForUserError),
        },
        _ => (),
    }

    // updating user info
    db.query(
        &format!(
            "\
						UPDATE user SET \
								last_ip = ?1, \
								last_login = ?2, \
								logins_count = logins_count + 1, \
								{}_device_id = ?3 \
								WHERE pkey_id = ?0;\
						",
            service.to_string()
        ),
        &[
            &pkey_id,
            &req.ip_address,
            &unix_timestamp(),
            &format!(r#"--force-string{}"#, req.device_id),
        ],
    )
    .await?;

    Ok(FunAuthAuthenticateResp {
        rows: vec![FunAuthAuthenticateRespRow {
            user_id: pkey_id,
            user_public_id: public_id,
        }],
    })
}

pub async fn fun_auth_set_token(
    db: &LocalDbClient,
    req: FunAuthSetTokenReq,
) -> Result<FunAuthSetTokenResp, RepositoryError> {
    // looking up user
    let mut user_auth_payload = db
        .query(
            "\
						SELECT is_blocked \
								FROM user \
								WHERE pkey_id = ?0;\
						",
            &[&req.user_id],
        )
        .await?
        .try_next_select()?;

    // checking known user and row count
    match user_auth_payload.rows.len() {
        1 => (),
        0 => return Err(RepositoryError::UnknownUserSetTokenError),
        _ => return Err(RepositoryError::RowCountDiagnosticError),
    };

    // checking block status
    let blocked = user_auth_payload
        .try_first_row()?
        .try_first_value()?
        .try_bool()?;

    if blocked {
        return Err(RepositoryError::BlockedUserSetTokenError);
    }

    // checking if service is permitted
    let service = EnumService::try_from(req.service_code)
        .map_err(|()| RepositoryError::ParseEnumServiceError)?;

    let token = match service {
        EnumService::Admin => req.admin_token,
        EnumService::User => req.user_token,
        _ => return Err(RepositoryError::InvalidServiceForSetTokenError),
    };

    // updating token
    db.query(
        &format!(
            "\
						UPDATE user SET \
								{}_token = ?0 \
								WHERE pkey_id = ?1;\
						",
            service.to_string()
        ),
        &[&token, &req.user_id],
    )
    .await?;

    Ok(FunAuthSetTokenResp {
        rows: vec![FunAuthSetTokenRespRow {}],
    })
}

pub async fn fun_auth_authorize(
    db: &LocalDbClient,
    mut snowflake: ConcurrentSnowflake,
    req: FunAuthAuthorizeReq,
) -> Result<FunAuthAuthorizeResp, RepositoryError> {
    // looking up user
    let user_auth_payload = db
        .query(
            "\
						SELECT pkey_id, user_token, admin_token, user_role \
								FROM user \
								WHERE username = ?0;\
						",
            &[&req.username],
        )
        .await?
        .try_next_select()?;

    let pkey_id = user_auth_payload.rows[0][0].try_i64()?;
    let user_token = Uuid::from_str(
        &user_auth_payload.rows[0][1]
            .possible_string()?
            .unwrap_or_default(),
    )
    .unwrap_or_default();

    let admin_token = Uuid::from_str(
        &user_auth_payload.rows[0][2]
            .possible_string()?
            .unwrap_or_default(),
    )
    .unwrap_or_default();

    let role: EnumRole = user_auth_payload.rows[0][3]
        .try_string()?
        .parse()
        .map_err(|_| RepositoryError::ParseEnumRoleError)?;

    // checking if service is permitted and token ok
    let (service, token_ok) = match req.service {
        EnumService::Admin => (req.service.to_string(), req.token == admin_token),
        EnumService::User => (req.service.to_string(), req.token == user_token),
        _ => return Err(RepositoryError::InvalidServiceForAuthorizeError),
    };

    // logging authorization attempt
    db.query(
        "\
				INSERT INTO authorization_attempt (\
						pkey_id, \
						fkey_user, \
						ip_address, \
						is_token_ok, \
						moment) \
						VALUES (\
								?0, \
								?1, \
								?2, \
								?3, \
								?4);\
				",
        &[
            &snowflake.gen()?,
            &pkey_id,
            &req.ip_address,
            &token_ok,
            &unix_timestamp(),
        ],
    )
    .await?;

    if !token_ok {
        return Err(RepositoryError::InvalidTokenError);
    }

    // checking known user and row count
    match user_auth_payload.rows.len() {
        1 => (),
        0 => return Err(RepositoryError::UnknownUserAuthorizeError),
        _ => return Err(RepositoryError::RowCountDiagnosticError),
    };

    // updating device info
    db.query(
        &format!(
            "\
						UPDATE user SET \
								{}_device_id = ?1 \
								WHERE pkey_id = ?0 \
								AND {}_token = ?2;\
						",
            service.to_string(),
            service.to_string(),
        ),
        &[
            &pkey_id,
            &format!(r#"--force-string{}"#, req.device_id),
            &req.token,
        ],
    )
    .await?;

    Ok(FunAuthAuthorizeResp {
        rows: vec![FunAuthAuthorizeRespRow {
            user_id: pkey_id,
            role: role,
        }],
    })
}

pub async fn _fun_auth_change_password(
    db: &LocalDbClient,
    mut snowflake: ConcurrentSnowflake,
    req: FunAuthChangePasswordReq,
) -> Result<FunAuthChangePasswordResp, RepositoryError> {
    // looking up user
    let user_auth_row = db
        .query(
            "\
						SELECT pkey_id, password_hash, is_blocked \
								FROM user \
								WHERE username = ?0;\
						",
            &[&req.username],
        )
        .await?
        .try_next_select()?
        .maybe_first_row()
        .ok_or_else(|| RepositoryError::_UnknownUserChangePasswordError)?;

    let pkey_id = user_auth_row[0].try_i64()?;
    let hash = user_auth_row[1].try_bytea()?;
    let blocked = user_auth_row[2].try_bool()?;

    // checking password ok and registering login attempt
    let password_ok = req.old_password_hash == hash;
    db.query(
        "\
				INSERT INTO login_attempt (\
					pkey_id, \
					fkey_user, \
					username, \
					password_hash, \
					ip_address, \
					device_id,
					device_os,
					is_password_ok, \
					moment) \
					VALUES (\
							?0, \
							?1, \
							?2, \
							?3, \
							?4, \
							?5, \
							?6, \
							?7, \
							?8);\
				",
        &[
            &snowflake.gen()?,
            &pkey_id,
            &req.username,
            &req.old_password_hash,
            &req.ip_address,
            &format!("--force-string{}", req.device_id),
            &req.device_os,
            &password_ok,
            &unix_timestamp(),
        ],
    )
    .await?;
    if !password_ok {
        return Err(RepositoryError::InvalidPasswordError);
    }

    // checking block status
    if blocked {
        return Err(RepositoryError::_BlockedUserChangePasswordError);
    }

    // updating user info
    db.query(
        "\
				UPDATE user SET \
						password_hash = ?1, \
						WHERE pkey_id = ?0;\
				",
        &[&pkey_id, &req.new_password_hash],
    )
    .await?;

    Ok(FunAuthChangePasswordResp {
        rows: vec![FunAuthChangePasswordRespRow {}],
    })
}

pub async fn _fun_get_recovery_question_data(
    db: &LocalDbClient,
    _req: FunGetRecoveryQuestionDataReq,
) -> Result<FunGetRecoveryQuestionDataResp, RepositoryError> {
    // getting every possible question?
    // TODO: understand why this query exists
    let recovery_question_payload = db
        .query(
            "\
						SELECT pkey_id, content, category \
								FROM recovery_question_data\
						",
            &[],
        )
        .await?
        .try_next_select()?;

    // collecting questions
    let mut rows: Vec<FunGetRecoveryQuestionDataRespRow> = Vec::new();
    for row in recovery_question_payload.rows {
        let id: i64 = row[0].try_i64()?;
        let content: String = row[1].try_string()?;
        let category: EnumRecoveryQuestionCategory = row[2]
            .try_string()?
            .parse()
            .map_err(|_| RepositoryError::_ParseEnumRecoveryQuestionCategoryError)?;
        rows.push(FunGetRecoveryQuestionDataRespRow {
            question_id: id,
            content: content,
            category: category,
        });
    }

    Ok(FunGetRecoveryQuestionDataResp { rows: rows })
}

pub async fn _fun_auth_set_recovery_questions(
    db: &LocalDbClient,
    mut snowflake: ConcurrentSnowflake,
    req: FunAuthSetRecoveryQuestionsReq,
) -> Result<FunAuthSetRecoveryQuestionsResp, RepositoryError> {
    // opening set questions transaction
    let affected_rows = db
        .query(
            // BUG: GlueSQL's transaction will save changes on COMMIT even if
            // some statements threw an error
            // TODO: update library and remove multi-request transaction when bug is fixed
            "\
						BEGIN;\
						DELETE FROM recovery_question \
								WHERE fkey_user = ?0;\
						",
            &[&req.user_id],
        )
        .await?
        .try_next_delete()?;

    // checking user had recovery questions
    if affected_rows == 0 {
        db.query("ROLLBACK;", &[]).await?;
        return Err(RepositoryError::_NoRowsAffectedDiagnosticError);
    }

    let mut statements: String = String::new();
    let mut tokens: Vec<String> = vec![req.user_id.to_string()];
    let mut token_number: i32 = 1;
    for (idx, question) in req.question_ids.into_iter().enumerate() {
        // constructing query string
        statements.push_str(&format!(
            "\
						INSERT INTO recovery_question (\
								fkey_user, \
								pkey_id, \
								fkey_question, \
								answer) \
								VALUES (\
										?0, \
										?{}, \
										?{}, \
										?{});\
						",
            token_number,
            token_number + 1,
            token_number + 2,
        ));
        // collecting tokens
        tokens.push(snowflake.gen()?.to_string());
        tokens.push(question.to_string());
        tokens.push(req.answers[idx].to_string());
        token_number += 3;
    }

    // inserting tokens in query
    let tokenized_statement =
        localdb::db::statements::tokenizer::tokenize_statements(&statements, tokens)
            .map_err(|_| RepositoryError::_TokenzationError)?;

    // executing query
    let payloads = db.query(&tokenized_statement, &[]).await;

    // checking all questions were set
    match payloads {
        Ok(_) => (),
        Err(e) => {
            db.query("ROLLBACK;", &[]).await?;
            return Err(e.into());
        }
    }

    // closing transaction
    db.query("COMMIT;", &[]).await?;

    Ok(FunAuthSetRecoveryQuestionsResp {
        rows: vec![FunAuthSetRecoveryQuestionsRespRow {}],
    })
}

pub async fn _fun_auth_basic_authenticate(
    db: &LocalDbClient,
    mut snowflake: ConcurrentSnowflake,
    req: FunAuthBasicAuthenticateReq,
) -> Result<FunAuthBasicAuthenticateResp, RepositoryError> {
    // looking up user
    let user_auth_row = db
        .query(
            "\
						SELECT pkey_id, is_blocked \
								FROM user \
								WHERE username = ?0;\
						",
            &[&req.username],
        )
        .await?
        .try_next_select()?
        .maybe_first_row()
        .ok_or_else(|| RepositoryError::UnknownUserLoginError)?;

    // registering login attempt
    let pkey_id = user_auth_row[0].try_i64()?;
    db.query(
        "\
				INSERT INTO login_attempt (\
					pkey_id, \
					fkey_user, \
					username, \
					password_hash, \
					ip_address, \
					device_id,
					device_os,
					moment) \
					VALUES (\
							?0, \
							?1, \
							?2, \
							?3, \
							?4, \
							?5, \
							?6, \
							?7);\
				",
        &[
            &snowflake.gen()?,
            &pkey_id,
            &req.username,
            &"",
            &req.ip_address,
            // flagging device id to override /src/localdb's token formatter
            // it currently parses string as numbers if only numerical by default
            // TODO: SOC completely broken, change soon
            &format!("--force-string{}", req.device_id),
            &req.device_os,
            &unix_timestamp(),
        ],
    )
    .await?;

    // checking block status
    let blocked = user_auth_row[1].try_bool()?;
    if blocked {
        return Err(RepositoryError::BlockedUserLoginError);
    }

    Ok(FunAuthBasicAuthenticateResp {
        rows: vec![FunAuthBasicAuthenticateRespRow {
            user_id: req.ip_address,
        }],
    })
}

pub async fn _fun_auth_get_recovery_questions(
    db: &LocalDbClient,
    req: FunAuthGetRecoveryQuestionsReq,
) -> Result<FunAuthGetRecoveryQuestionsResp, RepositoryError> {
    // getting questions
    let questions_payload = db
        .query(
            "\
						SELECT qd.pkey_id, qd.content \
								FROM recovery_question_data qd \
								JOIN recovery_question q \
								ON qd.pkey_id = q.fkey_question \
								WHERE q.fkey_user = ?0;\
						",
            &[&req.user_id],
        )
        .await?
        .try_next_select()?;

    // collecting questions
    let mut rows: Vec<FunAuthGetRecoveryQuestionsRespRow> = Vec::new();
    for row in questions_payload.rows {
        rows.push(FunAuthGetRecoveryQuestionsRespRow {
            question_id: row[0].try_i64()?,
            question: row[1].try_string()?,
        });
    }

    Ok(FunAuthGetRecoveryQuestionsResp { rows: rows })
}

pub async fn _fun_submit_recovery_answers(
    db: &LocalDbClient,
    req: FunSubmitRecoveryAnswersReq,
) -> Result<FunSubmitRecoveryAnswersResp, RepositoryError> {
    // getting answers
    let answers_payload = db
        .query(
            "\
						SELECT fkey_question, answer \
								FROM recovery_question \
								WHERE q.fkey_user = ?0;\
						",
            &[&req.user_id],
        )
        .await?
        .try_next_select()?;

    // checking number of answers provided
    if answers_payload.rows.len() != req.question_ids.len() {
        return Err(RepositoryError::_MustSubmitAllRecoveryQuestionsError);
    };

    // checking correct answers
    for row in answers_payload.rows {
        let question_id = row[0].try_i64()?;
        let idx = req
            .question_ids
            .iter()
            .position(|&q| (q as i64) == question_id)
            .ok_or(RepositoryError::_MustSubmitAllRecoveryQuestionsError)?;

        if req.answers[idx] != row[1].try_string()? {
            return Err(RepositoryError::_WrongRecoveryAnswersError);
        };
    }

    // updating user info
    db.query(
        "\
				UPDATE user SET \
						password_reset_token = ?1, \
						reset_token_valid = ?2, \
						WHERE pkey_id = ?0;\
				",
        &[&req.user_id, &req.password_reset_token, &req.token_valid],
    )
    .await?;

    Ok(FunSubmitRecoveryAnswersResp {
        rows: vec![FunSubmitRecoveryAnswersRespRow {}],
    })
}

pub async fn _fun_auth_reset_password(
    db: &LocalDbClient,
    req: FunAuthResetPasswordReq,
) -> Result<FunAuthResetPasswordResp, RepositoryError> {
    // opening reset transaction
    let affected_rows = db
        .query(
            // BUG: GlueSQL's transaction will save changes on COMMIT even if
            // some statements threw an error
            // TODO: update library and remove multi-request transaction when bug is fixed
            "\
						BEGIN; \
						UPDATE user SET \
								password_hash = ?1, \
								password_salt = ?2, \
								password_reset_token = NULL, \
								reset_token_valid = NULL, \
								WHERE pkey_id = ?0 \
								AND password_reset_token = ?3 \
								AND reset_token_valid > ?4;\
						",
            &[
                &req.user_id,
                &req.new_password_hash,
                &req.new_password_salt,
                &req.reset_token,
                &unix_timestamp(),
            ],
        )
        .await?
        .try_next_update()?;

    // checking only one reset token exists
    if affected_rows != 1 {
        db.query("ROLLBACK;", &[]).await?;
        return Err(RepositoryError::_InvalidRecoveryTokenError);
    }

    // closing transaction
    db.query("COMMIT;", &[]).await?;

    Ok(FunAuthResetPasswordResp {
        rows: vec![FunAuthResetPasswordRespRow {}],
    })
}

fn unix_timestamp() -> i64 {
    // seconds since january 1st, 1970
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("cannot login unknown user")]
    UnknownUserLoginError,
    #[error("cannot authorize unknown user")]
    UnknownUserAuthorizeError,
    #[error("cannot set token of unknown user")]
    UnknownUserSetTokenError,
    #[error("cannot get salt of unknown user")]
    UnknownUserGetSaltError,
    #[error("cannot change password of unknown user")]
    _UnknownUserChangePasswordError,
    #[error("cannot login blocked user")]
    BlockedUserLoginError,
    #[error("cannot set token of blocked user")]
    BlockedUserSetTokenError,
    #[error("cannot change password of blocked user")]
    _BlockedUserChangePasswordError,
    #[error("invalid role for admin service")]
    InvalidRoleForAdminError,
    #[error("invalid role for user service")]
    InvalidRoleForUserError,
    #[error("invalid password")]
    InvalidPasswordError,
    #[error("invalid token")]
    InvalidTokenError,
    #[error("invalid service for set token")]
    InvalidServiceForSetTokenError,
    #[error("invalid service for authorize")]
    InvalidServiceForAuthorizeError,
    #[error("missing recovery questions")]
    _MustSubmitAllRecoveryQuestionsError,
    #[error("incorrect recovery answers")]
    _WrongRecoveryAnswersError,
    #[error("invalid recovery token")]
    _InvalidRecoveryTokenError,
    #[error("diagnostic: invalid row count")]
    RowCountDiagnosticError,
    #[error("diagnostic: no rows affected")]
    _NoRowsAffectedDiagnosticError,
    #[error("localdb client failed")]
    LocalDbClientError(#[from] LocalDbClientError),
    #[error("concurrent snowflake failed")]
    ConcurrentSnowflakeError(#[from] ConcurrentSnowflakeError),
    #[error("failed to parse role string to enum")]
    ParseEnumRoleError,
    #[error("failed to parse recovery question category string to enum")]
    _ParseEnumRecoveryQuestionCategoryError,
    #[error("failed to parse service number to enum")]
    ParseEnumServiceError,
    #[error("failed to parse payload")]
    ParsePayloadError(#[from] ParsePayloadError),
    #[error("failed to parse select payload")]
    ParseSelectPayloadError(#[from] ParseSelectPayloadError),
    #[error("failed to parse row")]
    ParseRowError(#[from] ParseRowError),
    #[error("failed to parse value")]
    ParseValueError(#[from] ParseValueError),
    #[error("failed to parse uuid")]
    ParseUuidError(#[from] uuid::Error),
    #[error("failed to tokenize statements")]
    _TokenzationError,
}
