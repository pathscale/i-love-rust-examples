CREATE SCHEMA IF NOT EXISTS tbl;CREATE TYPE tbl.enum_role AS ENUM ('guest', 'user', 'admin', 'developer');
CREATE TYPE tbl.enum_recovery_question_category AS ENUM ('childhood', 'education', 'family', 'favorite', 'first', 'personal', 'pet', 'work', 'historical');
CREATE TYPE tbl.enum_service AS ENUM ('auth', 'user', 'admin');
