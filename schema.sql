CREATE TABLE users(
    email VARCHAR(128) PRIMARY KEY NOT NULL,
    username VARCHAR(128) NOT NULL,
    password_hash BINARY(4) NOT NULL,
    password_salt BINARY(16) NOT NULL
);

CREATE TABLE issues(
    uuid CHAR(36) PRIMARY KEY NOT NULL,
    author VARCHAR(128) NOT NULL,
    title VARCHAR(128) NOT NULL,
    FOREIGN KEY (author) REFERENCES users(email)
);

CREATE TABLE comments(
    uuid CHAR(32) PRIMARY KEY NOT NULL,
    author VARCHAR(128) NOT NULL,
    contents MEDIUMTEXT NOT NULL,
    timestamp BIGINT NOT NULL
);