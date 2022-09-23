use super::database;

pub struct Client {
	db: database::Database
}

impl Client {
	pub fn new(path: &str) -> Self {
		let mut new = Self{db: database::Database::new(path)};
		new
	}
}

impl Default for Client {
	fn default() -> Self {
		let mut default = Self{db: database::Database::default()};
		default
	}
}
