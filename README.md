# History tables

CREATE TABLE books (
id integer PRIMARY KEY AUTOINCREMENT,
name text Not null,
link text Not null,
description text Not null);

CREATE TABLE publications (
id integer PRIMARY KEY AUTOINCREMENT,
name text Not null,
link text Not null,
description text Not null);

CREATE TABLE texts (
id integer PRIMARY KEY AUTOINCREMENT,
name text Not null,
link text Not null,
description text Not null);
