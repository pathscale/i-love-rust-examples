pub const TABLES: [&str; 8] = [
    "
		CREATE TABLE IF NOT EXISTS user
		(
			pkey_id INTEGER PRIMARY KEY, 
			role TEXT NOT NULL DEFAULT 'user',
			public_id INTEGER NOT NULL,
			username TEXT UNIQUE NOT NULL,
			password_hash BYTEA NOT NULL,
			password_salt BYTEA NOT NULL,
			age INTEGER NOT NULL,
			preferred_language TEXT NOT NULL,
			family_name TEXT NULL,
			given_name TEXT NULL,
			agreed_tos BOOLEAN NOT NULL,
			agreed_privacy BOOLEAN NOT NULL,
			created_at INTEGER NOT NULL, -- unix timestamp
			updated_at INTEGER NOT NULL, -- unix timestamp
			email TEXT NOT NULL,
			phone_number TEXT NOT NULL,
			last_ip TEXT NOT NULL,
			last_login INTEGER NULL, -- unix timestamp
			last_password_reset INTEGER NULL, -- unix timestamp
			logins_count INTEGER NOT NULL DEFAULT 0,
			user_device_id TEXT NULL,
			admin_device_id TEXT NULL,
			password_reset_token UUID NULL,
			reset_token_valid UUID NULL,
			user_token UUID NULL,
			admin_token UUID NULL,
			is_blocked BOOLEAN NOT NULL DEFAULT false,
		);
	",
    "
		CREATE TABLE IF NOT EXISTS login_attempt
		(
			pkey_id INTEGER PRIMARY KEY,
			fkey_user INTEGER NULL, -- fkey to user table
			username TEXT NOT NULL,
			password_hash BYTEA NOT NULL,
			ip_address TEXT NOT NULL,
			device_id TEXT NULL,
			device_os TEXT NULL,
			is_password_ok BOOLEAN NULL,
			moment INTEGER NOT NULL, -- unix timestamp
		);
	",
    "
		CREATE TABLE IF NOT EXISTS authorization_attempt
		(
			pkey_id INTEGER PRIMARY KEY,
			fkey_user INTEGER NOT NULL, -- fkey to user table
			ip_address TEXT NOT NULL,
			is_token_ok BOOLEAN NOT NULL,
			moment INTEGER NOT NULL, -- unix timestamp
		);
	",
    "
		CREATE TABLE IF NOT EXISTS reset_password_attempt
		(
			pkey_id INTEGER PRIMARY KEY,
			fkey_user INTEGER NOT NULL, -- fkey to user table
			initiated_at INTEGER NOT NULL, -- unix timestamp
			valid_until INTEGER NOT NULL, -- initiated + 24 hours (86400)
			code TEXT NOT NULL,
		);
	",
    "
		CREATE TABLE IF NOT EXISTS recovery_question
		(
			pkey_id INTEGER PRIMARY KEY,
			fkey_user INTEGER NOT NULL, -- fkey to user table
			fkey_question INTEGER NOT NULL, -- fkey to recovery_question_data
			answer TEXT NOT NULL,
		);
	",
    "
		CREATE TABLE IF NOT EXISTS recovery_question_data
		(
			pkey_id INTEGER PRIMARY KEY,
			content TEXT NOT NULL,
			category TEXT NOT NULL,
		);
	",
    "
		CREATE TABLE IF NOT EXISTS support_ticket
		(
			pkey_id INTEGER PRIMARY KEY,
			fkey_user INTEGER NOT NULL, -- fkey to user table
			fkey_handler_user INTEGER NULL, -- fkey to user table
			content TEXT NOT NULL,
			response TEXT NOT NULL,
			created_at INTEGER NOT NULL, -- unix timestamp
			updated_at INTEGER NOT NULL, -- unix timestamp
		);
	",
    "
		CREATE TABLE IF NOT EXISTS bad_request
		(
			pkey_id INTEGER PRIMARY KEY,
			fkey_user INTEGER NULL, -- fkey to user table
			ip_address TEXT NOT NULL,
			method_code INTEGER NULL,
			error_code INTEGER NOT NULL,
			device_id TEXT NULL,
			device_os TEXT NULL,
			raw TEXT NULL,
			moment INTEGER NOT NULL, --unix timestamp
		);
	",
];

pub const INDEXES: [&str; 2] = [
    "CREATE INDEX IF NOT EXISTS uidx_user_username ON user (username);",
    "CREATE INDEX IF NOT EXISTS uidx_user_public_id ON user (public_id);",
];
