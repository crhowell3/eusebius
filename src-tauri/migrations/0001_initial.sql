CREATE TABLE IF NOT EXISTS categories (
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    tag  TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL
);

INSERT OR IGNORE INTO categories (tag, name) VALUES ('MISC', 'Miscellaneous');

CREATE TABLE IF NOT EXISTS deaths (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    first_name      TEXT NOT NULL,
    last_name       TEXT NOT NULL,
    date_of_death   TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS works (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    description     TEXT NOT NULL,
    category_id     INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (category_id) REFERENCES categories(id)
        ON UPDATE CASCADE
        ON DELETE SET DEFAULT
);

CREATE TABLE IF NOT EXISTS baptisms (
    family_id       TEXT PRIMARY KEY NOT NULL,
    last_name       TEXT NOT NULL,
    first_name      TEXT NOT NULL,
    date_baptized   TEXT NOT NULL,
    witness         TEXT NOT NULL,
    location        TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS families (
    family_id               TEXT PRIMARY KEY NOT NULL,
    mail_route              TEXT NOT NULL,
    first_name              TEXT NOT NULL,
    last_name               TEXT NOT NULL,
    is_member               INTEGER NOT NULL DEFAULT 0,
    is_active               INTEGER NOT NULL DEFAULT 0,
    date_of_birth           TEXT NOT NULL,
    anniversary_month       TEXT NOT NULL,
    anniversary_day         TEXT NOT NULL,
    home_phone              TEXT NOT NULL,
    cell_phone              TEXT NOT NULL,
    work_phone              TEXT NOT NULL,
    address                 TEXT NOT NULL,
    city                    TEXT NOT NULL,
    state                   TEXT NOT NULL,
    zip                     TEXT NOT NULL,
    email_address           TEXT NOT NULL,
    on_bulletin_email_list  INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS spouses (
    family_id               TEXT PRIMARY KEY NOT NULL,
    first_name              TEXT NOT NULL,
    last_name               TEXT NOT NULL,
    is_member               INTEGER NOT NULL DEFAULT 0,
    is_active               INTEGER NOT NULL DEFAULT 0,
    date_of_birth           TEXT,
    cell_phone              TEXT,
    work_phone              TEXT,
    email_address           TEXT,
    on_bulletin_email_list  INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (family_id) REFERENCES families(family_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS children (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    family_id               TEXT NOT NULL,
    first_name              TEXT NOT NULL,
    last_name               TEXT NOT NULL,
    is_member               INTEGER NOT NULL DEFAULT 0,
    is_active               INTEGER NOT NULL DEFAULT 0,
    date_of_birth           TEXT,
    cell_phone              TEXT,
    work_phone              TEXT,
    email_address           TEXT,
    on_bulletin_email_list  INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (family_id) REFERENCES families(family_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
