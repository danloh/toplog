-- Your SQL goes here

CREATE SEQUENCE IF NOT EXISTS serial_seq
  INCREMENT 1
  MAXVALUE 2147483647
  START 83645
  CACHE 1;

CREATE TABLE users (
  id INTEGER PRIMARY KEY DEFAULT nextval('serial_seq'),
  uname VARCHAR UNIQUE NOT NULL,
  psw_hash VARCHAR NOT NULL,
  join_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_seen TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  avatar VARCHAR NOT NULL DEFAULT '',
  email VARCHAR NOT NULL DEFAULT '',
  link VARCHAR NOT NULL DEFAULT '',
  intro TEXT NOT NULL DEFAULT '',
  location VARCHAR NOT NULL DEFAULT '',
  nickname VARCHAR NOT NULL DEFAULT '',
  permission SMALLINT NOT NULL DEFAULT 3,
  auth_from VARCHAR NOT NULL DEFAULT '',
  email_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
  karma INTEGER NOT NULL DEFAULT 100,
  is_pro BOOLEAN NOT NULL DEFAULT FALSE,
  can_push BOOLEAN NOT NULL DEFAULT FALSE,
  push_email VARCHAR NOT NULL DEFAULT '',
  UNIQUE (uname, email)
);