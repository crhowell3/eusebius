pub const ALL_TABLES: &[&str] = &[
    DEATHS_INIT,
    WORKS_INIT,
    BAPTISMS_INIT,
    FAMILIES_INIT,
    SPOUSES_INIT,
    CHILDREN_INIT,
];

const DEATHS_INIT: &'static str = "CREATE TABLE IF NOT EXISTS deaths (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    first_name      TEXT NOT NULL,
    last_name       TEXT NOT NULL,
    date_of_death   TEXT NOT NULL
)";

const WORKS_INIT: &'static str = "CREATE TABLE IF NOT EXISTS works (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    description     TEXT NOT NULL,
    category_tag    TEXT NOT NULL DEFAULT 'MISC',
    FOREIGN KEY (category_tag) REFERENCES categories(tag)
        ON UPDATE CASCADE
        ON DELETE SET DEFAULT
)";

const BAPTISMS_INIT: &'static str = "CREATE TABLE IF NOT EXISTS baptisms (
    family_id       TEXT PRIMARY KEY NOT NULL,
    last_name       TEXT NOT NULL,
    first_name      TEXT NOT NULL,
    date_baptized   TEXT NOT NULL,
    witness         TEXT NOT NULL,
    location        TEXT NOT NULL
)";

const FAMILIES_INIT: &'static str = "CREATE TABLE IF NOT EXISTS families (
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
)";

const SPOUSES_INIT: &'static str = "CREATE TABLE IF NOT EXISTS spouses (
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
)";

const CHILDREN_INIT: &'static str = "CREATE TABLE IF NOT EXISTS children (
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
)";
