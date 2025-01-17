This is mostly a basic CRUD app.

The cool part is an `Executable` trait that lets you execute database queries like this:
```rust
fn get_all_users() -> Vec<User> {
    "SELECT id, username FROM users".query_many()
}

fn update_user(id: String, patch: UserPatch) -> Option<User> {
    "UPDATE users SET username = ?2 WHERE id = ?1".execute_with_params(params![id, patch.username]);
    "SELECT id, username FROM users where id = ?1".query_one(params![id])
}
```
