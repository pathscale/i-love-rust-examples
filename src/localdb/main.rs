use std::sync::Arc;

use eyre::Result;
use futures::lock::Mutex;

use lib::ws::{WebsocketServer};
use lib::config::load_config;
use lib::log::setup_logs;

use iloverust::localdb::db::database::Database;
use iloverust::localdb::db::statements::tokenizer;
use iloverust::localdb::endpoints::endpoint_localdb_select;
use iloverust::localdb::method::QueryHandler;

#[tokio::main]
async fn main() -> Result<()> {
	let mut db = Database::default();

	println!("{:?}", db.exec(
			&tokenizer::tokenize_statements(
				"CREATE TABLE IF NOT EXISTS test
				(
					id INTEGER PRIMARY KEY,
					content TEXT NULL,
					other_id INTEGER NULL,
					floating FLOAT NULL,
				);",
				Vec::new(),
			)?
		)
	);

	println!("{:?}", db.exec(
			&tokenizer::tokenize_statements(
				"INSERT INTO test VALUES (?0, ?1);",
				vec!["0".to_owned(), "yes, that's right".to_owned()],
			)?
		)
	);

	println!("{:?}", db.exec(
			&tokenizer::tokenize_statements(
				"INSERT INTO test VALUES (?0, ?1, ?0)",
				vec!["1".to_owned(),"this really works!".to_owned()],
			)?
		)
	);

	println!("{:?}", db.exec(
			&tokenizer::tokenize_statements(
				"INSERT INTO test VALUES (?0, ?1, ?0, ?2)",
				vec!["2".to_owned(),"yes!".to_owned(),"0".to_owned()],
			)?
		)
	);

	println!("{:?}", db.exec(
			&tokenizer::tokenize_statements(
				"INSERT INTO test VALUES (?0, ?1, ?0, ?2)",
				vec!["3".to_owned(),"omg!".to_owned(),"1.".to_owned()],
			)?
		)
	);

	println!("{:?}", db.exec(
			&tokenizer::tokenize_statements(
				"INSERT INTO test VALUES (?0, ?1, ?0, ?2)",
				vec!["4".to_owned(),"omg!".to_owned(),"0.4214214".to_owned()],
			)?
		)
	);

	println!("{:?}", db.exec(
			&tokenizer::tokenize_statements(
				"INSERT INTO test VALUES (?3, ?1, ?2, ?0)",
				vec!["0.4214214".to_owned(), "omg!".to_owned(), "1".to_owned(), "5".to_owned()],
			)?
		)
	);

	println!("{:?}", db.exec(
			&tokenizer::tokenize_statements(
				"UPDATE test SET content = ?0 WHERE id = ?1;",
				vec!["new content".to_owned(), "0".to_owned()],
			)?
		)
	);

	println!("{:?}", db.exec(
			&tokenizer::tokenize_statements(
				"SELECT * FROM test LIMIT 2 OFFSET 2;",
				Vec::new(),
			)?
		)
	);

	let config = load_config("localdb".to_owned())?;
	setup_logs(config.app.log_level)?;

	let mut server = WebsocketServer::new(config.app);
	server.add_handler(endpoint_localdb_select(), QueryHandler{db: Arc::new(Mutex::new(db))});
	server.listen().await?;
	Ok(())
}
