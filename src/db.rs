use crate::users::{CreateUser, User};
use lazy_static::lazy_static;
use rusqlite::{params, Connection, Error, OptionalExtension, Row};
use std::sync::Mutex;
use uuid::Uuid;

lazy_static! {
    pub static ref DB_CONN: Mutex<Connection> = Mutex::new(Connection::open("users.db").unwrap());
}

const SELECT_ALL_USERS: &'static str = "SELECT id, username FROM users";
const SELECT_USER_BY_ID: &'static str = "SELECT id, username FROM users where id = ?1";
const INSERT_USER: &'static str = "INSERT INTO users (id, username) VALUES (?1, ?2)";

static MAP_ROW_TO_USER: fn(&Row) -> Result<User, Error> = |row: &Row| {
    Ok::<User, Error>(User {
        id: row.get(0)?,
        username: row.get(1)?,
    })
};

pub fn init_db() {
    let conn = DB_CONN.lock().unwrap();
    conn.execute("DROP TABLE IF EXISTS users", []).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL
        )",
        [],
    )
    .unwrap();

    conn.execute(INSERT_USER, params![1, "John"]).unwrap();
}

pub fn get_user(id: String) -> Option<User> {
    let conn = DB_CONN.lock().unwrap();
    let mut stmt = conn.prepare(SELECT_USER_BY_ID).unwrap();

    stmt.query_row(params![String::from(id)], MAP_ROW_TO_USER)
        .optional()
        .unwrap()
}

pub fn get_all_users() -> Vec<User> {
    let conn = DB_CONN.lock().unwrap();
    let mut stmt = conn.prepare(SELECT_ALL_USERS).unwrap();

    stmt.query_map(params![], MAP_ROW_TO_USER)
        .unwrap()
        .collect::<rusqlite::Result<Vec<User>>>()
        .unwrap()
}

pub fn insert_user(user: CreateUser) -> User {
    let user = User {
        id: String::from(Uuid::new_v4()),
        username: user.username,
    };

    let conn = DB_CONN.lock().unwrap();
    conn.execute(INSERT_USER, params![user.id, user.username])
        .unwrap();

    user
}
