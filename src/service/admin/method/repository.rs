use thiserror::Error;

use lib::database::LocalDbClient;
use localdb::parsetools::*;

use gen::database::*;
use gen::model::EnumRole;

pub async fn fun_admin_list_users(
    db: &LocalDbClient,
    req: FunAdminListUsersReq,
) -> Result<FunAdminListUsersResp, RepositoryError> {
    let users_payload = db
        .query(
            "\
						SELECT pkey_id, public_id, email, username, user_role, created_at, updated_at \
								FROM user \
								ORDER BY pkey_id \
								OFFSET ?0 \
								LIMIT ?1; \
						",
            &[&req.offset, &req.limit],
        )
        .await?
        .try_next_select()?;

    let mut rows: Vec<FunAdminListUsersRespRow> = Vec::new();
    for row in users_payload.rows {
        let role: EnumRole = row[4]
            .try_string()?
            .parse()
            .map_err(|_| RepositoryError::ParseEnumRoleError)?;

        rows.push(FunAdminListUsersRespRow {
            user_id: row[0].try_i64()?,
            user_public_id: row[1].try_i64()?,
            email: row[2].try_string()?,
            username: row[3].try_string()?,
            role: role,
            updated_at: row[5].try_i64()?,
            created_at: row[6].try_i64()?,
        });
    }

    Ok(FunAdminListUsersResp { rows: rows })
}

pub async fn fun_admin_assign_role(
    db: &LocalDbClient,
    req: FunAdminAssignRoleReq,
) -> Result<FunAdminAssignRoleResp, RepositoryError> {
    let operator_role: EnumRole = db
        .query(
            "\
						SELECT user_role \
								FROM user \
								WHERE pkey_id = ?0;\
						",
            &[&req.operator_user_id],
        )
        .await?
        .try_next_select()?
        .try_first_row()?
        .try_first_value()?
        .try_string()?
        .parse()
        .map_err(|_| RepositoryError::ParseEnumRoleError)?;

    match operator_role {
        EnumRole::Admin => (),
        _ => return Err(RepositoryError::InvalidRoleForAssignRoleError),
    };

    let affected_rows = db
        .query(
            "\
				UPDATE user SET user_role = ?0 \
						WHERE public_id = ?1; \
				",
            &[&req.new_role.to_string(), &req.user_public_id],
        )
        .await?
        .try_next_update()?;

    if affected_rows == 0 {
        return Err(RepositoryError::NoRowsAffectedDiagnosticError);
    };

    Ok(FunAdminAssignRoleResp {
        rows: vec![FunAdminAssignRoleRespRow {}],
    })
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("invalid role for assign role")]
    InvalidRoleForAssignRoleError,
    #[error("diagnostic: no rows affected")]
    NoRowsAffectedDiagnosticError,
    #[error("localdb client failed")]
    LocalDbClientError(#[from] lib::database::LocalDbClientError),
    #[error("failed to parse role string to enum")]
    ParseEnumRoleError,
    #[error("failed to parse payload")]
    ParsePayloadError(#[from] ParsePayloadError),
    #[error("failed to parse select payload")]
    ParseSelectPayloadError(#[from] ParseSelectPayloadError),
    #[error("failed to parse row")]
    ParseRowError(#[from] ParseRowError),
    #[error("failed to parse value")]
    ParseValueError(#[from] ParseValueError),
}
