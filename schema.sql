CREATE TABLE IF NOT EXISTS todos (
    id          INTEGER PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    done        BOOLEAN NOT NULL DEFAULT FALSE
);


CREATE TABLE IF NOT EXISTS users (
    id          INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT NOT NULL,
    password TEXT NOT NULL
);
