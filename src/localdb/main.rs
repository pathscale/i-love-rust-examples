use iloverust::localdb::client;
fn main() {
	let mut client = client::Client::default();

	let write_queries = [
		"CREATE TABLE IF NOT EXISTS test (id INTEGER, content TEXT NULL);",
		"INSERT INTO test (id) VALUES (0);",
		"INSERT INTO test (id) VALUES (1);",
		"INSERT INTO test (id) VALUES (2);",
		"INSERT INTO test VALUES (3, 'yes thats right');",
		"UPDATE test SET content = 'new content' WHERE id = 0;",
	];

	let read_queries = [
		"SELECT * FROM test;"
	];

	for sql in write_queries {
		let result = client.write(sql);
		println!("{:?}", result);
	}

	for sql in read_queries {
		let result = client.read(sql);
		println!("{:?}", result);
	}
}
