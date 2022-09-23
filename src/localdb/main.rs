use gluesql::prelude::{Glue, SledStorage};

fn main() {
	let users_table = 
	"
		CREATE TABLE IF NOT EXISTS user
		(
			pkey_id INTEGER PRIMARY KEY, 
			role TEXT NOT NULL DEFAULT 'user',
			public_id INTEGER NOT NULL,
			username TEXT NOT NULL,
			password_hash BYTEA NOT NULL,
			password_salt BYTEA NOT NULL,
			age INTEGER NOT NULL,
			oreferred_language TEXT NOT NULL,
			family_name TEXT NULL,
			given_name TEXT NULL,
			agreed_tos BOOLEAN NOT NULL,
			agreed_privacy BOOLEAN NOT NULL,
			created_at TIMESTAMP NOT NULL,
			updated_at TIMESTAMP NOT NULL,
			email TEXT NOT NULL,
			phone_number TEXT NOT NULL,
			last_ip TEXT NOT NULL,
			last_login TIMESTAMP NULL,
			last_password_reset TIMESTAMP NULL,
			logins_count INTEGER NOT NULL DEFAULT 0,
			user_device_id TEXT NOT NULL,
			admin_device_id TEXT NOT NULL,
			password_reset_token UUID NULL,
			reset_token_valid UUID NULL,
			user_token UUID NULL,
			admin_token UUID NULL,
			is_blocked BOOLEAN NOT NULL DEFAULT false,
		);
	";

	let storage = SledStorage::new("storage").unwrap();
	let mut glue = Glue::new(storage);
	let output = glue.execute(users_table);
	println!("{:?}", output);
}
