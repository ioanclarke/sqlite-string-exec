use crate::users::{User, UserCreate, UserPatch};
use lazy_static::lazy_static;
use rusqlite::{params, Connection, Error, OptionalExtension, Params, Row, ToSql};
use std::sync::{Mutex, MutexGuard};

pub fn init_db() {
    DROP_USER_TABLE.execute();
    CREATE_USER_TABLE.execute();
    INSERT_USER.execute_with_params(params![1, "John"]);
}

pub fn insert_user(user: UserCreate) -> User {
    let user = User::new(user.username);
    INSERT_USER.execute_with_params(params![user.id, user.username]);
    user
}

pub fn get_all_users() -> Vec<User> {
    SELECT_ALL_USERS.query_many()
}

pub fn get_user(id: String) -> Option<User> {
    SELECT_USER_BY_ID.query_one(params![id])
}

pub fn update_user(id: String, patch: UserPatch) -> Option<User> {
    UPDATE_USER_BY_ID.execute_with_params(params![id, patch.username]);
    SELECT_USER_BY_ID.query_one(params![id])
}

lazy_static! {
    pub static ref DB_CONN: Mutex<Connection> = Mutex::new(Connection::open("users.db").unwrap());
}

fn get_conn() -> MutexGuard<'static, Connection> {
    DB_CONN.lock().unwrap()
}

const DROP_USER_TABLE: &str = "DROP TABLE IF EXISTS users";
const CREATE_USER_TABLE: &str = "CREATE TABLE IF NOT EXISTS users (
                                    id TEXT PRIMARY KEY,
                                    username TEXT NOT NULL
                                )";
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

trait Executable<T> {
    fn execute(&self);
    fn execute_with_params<P>(&self, params: P)
    where
        P: IntoIterator + Params,
        P::Item: ToSql;
    fn query_many(&self) -> Vec<T>;
    fn query_one<P>(&self, params: P) -> Option<T>
    where
        P: IntoIterator + Params,
        P::Item: ToSql;
}

impl Executable<User> for &str {
    fn execute(&self) {
        let conn = get_conn();
        conn.execute(&self, []).unwrap();
    }

    fn execute_with_params<P>(&self, params: P)
    where
        P: IntoIterator + Params,
        P::Item: ToSql,
    {
        let conn = get_conn();
        conn.execute(&self, params).unwrap();
    }

    fn query_many(&self) -> Vec<User> {
        let conn = get_conn();
        let mut stmt = conn.prepare(SELECT_ALL_USERS).unwrap();

        stmt.query_map(params![], MAP_ROW_TO_USER)
            .unwrap()
            .collect::<rusqlite::Result<Vec<User>>>()
            .unwrap()
    }
    fn query_one<P>(&self, params: P) -> Option<User> 
    where
        P: IntoIterator + Params,
        P::Item: ToSql,
    {
        let conn = DB_CONN.lock().unwrap();
        let mut stmt = conn.prepare(SELECT_USER_BY_ID).unwrap();

        stmt.query_row(params, MAP_ROW_TO_USER)
            .optional()
            .unwrap()
    }
}
