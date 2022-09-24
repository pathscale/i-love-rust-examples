use iloverust::localdb::client;
fn main() {
	let mut client = client::Client::default();

	println!("{:?}", client.write(
			"CREATE TABLE IF NOT EXISTS test
			(
				id INTEGER PRIMARY KEY,
				content TEXT NULL,
				other_id INTEGER NULL,
				floating FLOAT NULL,
			);",
			Vec::new(),
		)
	);

	println!("{:?}", client.write(
			"INSERT INTO test (id) VALUES (?0);",
			vec!["0".to_owned()],
		)
	);

	println!("{:?}", client.write(
			"INSERT INTO test (id) VALUES (?126);",
			vec!["1".to_owned()],
		)
	);

	println!("{:?}", client.write(
			"INSERT INTO test (id) VALUES (?432214124);",
			vec!["2".to_owned()],
		)
	);

	println!("{:?}", client.write(
			"INSERT INTO test VALUES (?77, ?64);",
			vec!["3".to_owned(), "yes, that's right".to_owned()],
		)
	);

	println!("{:?}", client.write(
			"INSERT INTO test VALUES (?0, ?1, ?0)",
			vec!["5".to_owned(),"this really works!".to_owned()],
		)
	);

	println!("{:?}", client.write(
			"INSERT INTO test VALUES (?3, ?25, ?3, ?156)",
			vec!["6".to_owned(),"yes!".to_owned(),"0".to_owned()],
		)
	);

	println!("{:?}", client.write(
		"INSERT INTO test VALUES (?2, ?25, ?2, ?256)",
		vec!["7".to_owned(),"omg!".to_owned(),"1.".to_owned()],
		)
	);

	println!("{:?}", client.write(
		"INSERT INTO test VALUES (?2, ?25, ?2, ?256)",
		vec!["8".to_owned(),"omg!".to_owned(),"0.4214214".to_owned()],
		)
	);

	println!("{:?}", client.write(
			"UPDATE test SET content = ?2983 WHERE id = ?2;",
			vec!["new content".to_owned(), "0".to_owned()],
		)
	);

	println!("{:?}", client.read(
			"SELECT * FROM test;",
			Vec::new(),
		)
	);
}
