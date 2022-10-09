use lib::database::LocalDbClient;

pub async fn fun_user_foo(_: &LocalDbClient, _: ()) -> Result<(), RepositoryError> {
    Ok(())
}

#[derive(Debug)]
pub enum RepositoryError {
    FooError(&'static str),
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FooError(e) => write!(f, "{:?}", e),
        }
    }
}

impl std::error::Error for RepositoryError {}
