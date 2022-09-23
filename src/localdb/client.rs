use gluesql::prelude::{Glue, SledStorage};
use gluesql::core::executor;

pub struct Client {
	inner: Glue<SledStorage>,
}

impl Client {
	pub fn new(path: &str) -> Self {
		let storage = SledStorage::new(path).unwrap();		
		Self{inner: Glue::new(storage)}
	}

	pub fn query(&mut self, query: &str) -> Vec<executor::Payload> {
		self.inner.execute(query).unwrap()
	}
}

impl Default for Client {
	fn default() -> Self {
		let storage = SledStorage::new("storage").unwrap();		
		Self{inner: Glue::new(storage)}
	}
}
