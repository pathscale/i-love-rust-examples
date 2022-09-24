use iloverust::localdb::client;
fn main() {
	let mut client = client::Client::default();

	println!("{:?}", client.write(
			"CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, content TEXT NULL);",
			Vec::new(),
		)
	);

	println!("{:?}", client.write(
			"INSERT INTO test (id) VALUES (?);",
			vec!["0".to_owned()],
		)
	);

	println!("{:?}", client.write(
			"INSERT INTO test (id) VALUES (?);",
			vec!["1".to_owned()],
		)
	);

	println!("{:?}", client.write(
			"INSERT INTO test (id) VALUES (?);",
			vec!["2".to_owned()],
		)
	);

	println!("{:?}", client.write(
			"INSERT INTO test VALUES (?, ?);",
			vec!["3".to_owned(), "yes, that's right".to_owned()],
		)
	);

	println!("{:?}", client.write(
			"UPDATE test SET content = ? WHERE id = ?;",
			vec!["new content".to_owned(), "0".to_owned()],
		)
	);

	println!("{:?}", client.read(
			"SELECT * FROM test;",
			Vec::new(),
		)
	);
}
