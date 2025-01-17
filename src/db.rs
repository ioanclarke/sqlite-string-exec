use crate::users::{UserCreate, User, UserPatch};
use lazy_static::lazy_static;
use rusqlite::{params, Connection, Error, OptionalExtension, Row};
use std::sync::Mutex;
use uuid::Uuid;

lazy_static! {
    pub static ref DB_CONN: Mutex<Connection> = Mutex::new(Connection::open("users.db").unwrap());
}

const INSERT_USER: &str = "INSERT INTO users (id, username) VALUES (?1, ?2)";
const SELECT_ALL_USERS: &str = "SELECT id, username FROM users";
const SELECT_USER_BY_ID: &str = "SELECT id, username FROM users where id = ?1";
const UPDATE_USER_BY_ID: &str = "UPDATE users SET username = ?2 WHERE id = ?1";

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

pub fn insert_user(user: UserCreate) -> User {
    let conn = DB_CONN.lock().unwrap();
    
    let user = User {
        id: String::from(Uuid::new_v4()),
        username: user.username,
    };
    conn.execute(INSERT_USER, params![user.id, user.username])
        .unwrap();

    user
}

pub fn get_all_users() -> Vec<User> {
    let conn = DB_CONN.lock().unwrap();
    let mut stmt = conn.prepare(SELECT_ALL_USERS).unwrap();

    stmt.query_map(params![], MAP_ROW_TO_USER)
        .unwrap()
        .collect::<rusqlite::Result<Vec<User>>>()
        .unwrap()
}

pub fn get_user(id: String) -> Option<User> {
    let conn = DB_CONN.lock().unwrap();
    let mut stmt = conn.prepare(SELECT_USER_BY_ID).unwrap();

    stmt.query_row(params![String::from(id)], MAP_ROW_TO_USER)
        .optional()
        .unwrap()
}

pub fn update_user(id: String, patch: UserPatch) -> User {
    let conn = DB_CONN.lock().unwrap();
    
    conn.execute(UPDATE_USER_BY_ID, params![id, patch.username]).unwrap();

    let mut stmt = conn.prepare(SELECT_USER_BY_ID).unwrap();

    stmt.query_row(params![String::from(id)], MAP_ROW_TO_USER)
        .optional()
        .unwrap()
        .unwrap()
}
