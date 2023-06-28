# Test

CREATE TABLE books (
id integer PRIMARY KEY AUTOINCREMENT ,
name text Not null,
link text Not null,
description text Not null,
cover text Not null);

CREATE TABLE posts (
id integer PRIMARY KEY AUTOINCREMENT ,
title text Not null,
lead text Not null,
body text Not null,
cover text Not null,
created_at DATETIME DEFAULT CURRENT_TIMESTAMP);

CREATE TABLE publications (
id integer PRIMARY KEY AUTOINCREMENT ,
name text Not null,
link text Not null,
description text Not null,
cover text Not null);

CREATE TABLE textbooks (
id integer PRIMARY KEY AUTOINCREMENT ,
name text Not null,
link text Not null,
description text Not null,
cover text Not null);

CREATE TABLE texts (
id integer PRIMARY KEY AUTOINCREMENT ,
name text Not null,
link text Not null,
description text Not null,
cover text Not null);
