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
            .map_err(|_| RepositoryError::ParseEnumError("could not parse role string to enum"))?;

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
        .map_err(|_| RepositoryError::ParseEnumError("could not parse role string to enum"))?;

    match operator_role {
        EnumRole::Admin => (),
        _ => return Err(RepositoryError::InvalidRoleError("invalid role")),
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
        return Err(RepositoryError::DiagnosticError("user role not updated"));
    };

    Ok(FunAdminAssignRoleResp {
        rows: vec![FunAdminAssignRoleRespRow {}],
    })
}

#[derive(Debug)]
pub enum RepositoryError {
    InvalidRoleError(&'static str),
    DiagnosticError(&'static str),
    LocalDbClientError(lib::database::LocalDbClientError),
    ParseEnumError(&'static str),
    ParsePayloadError(ParsePayloadError),
    ParseSelectPayloadError(ParseSelectPayloadError),
    ParseRowError(ParseRowError),
    ParseValueError(ParseValueError),
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRoleError(e) => write!(f, "{:?}", e),
            Self::DiagnosticError(e) => write!(f, "{:?}", e),
            Self::LocalDbClientError(e) => write!(f, "{:?}", e),
            Self::ParseEnumError(e) => write!(f, "{:?}", e),
            Self::ParsePayloadError(e) => write!(f, "{:?}", e),
            Self::ParseSelectPayloadError(e) => write!(f, "{:?}", e),
            Self::ParseRowError(e) => write!(f, "{:?}", e),
            Self::ParseValueError(e) => write!(f, "{:?}", e),
        }
    }
}

impl std::error::Error for RepositoryError {}

impl From<lib::database::LocalDbClientError> for RepositoryError {
    fn from(e: lib::database::LocalDbClientError) -> Self {
        Self::LocalDbClientError(e)
    }
}

impl From<ParsePayloadError> for RepositoryError {
    fn from(e: ParsePayloadError) -> Self {
        Self::ParsePayloadError(e)
    }
}

impl From<ParseSelectPayloadError> for RepositoryError {
    fn from(e: ParseSelectPayloadError) -> Self {
        Self::ParseSelectPayloadError(e)
    }
}

impl From<ParseRowError> for RepositoryError {
    fn from(e: ParseRowError) -> Self {
        Self::ParseRowError(e)
    }
}

impl From<ParseValueError> for RepositoryError {
    fn from(e: ParseValueError) -> Self {
        Self::ParseValueError(e)
    }
}
