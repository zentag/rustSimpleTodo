use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Todo {
    id: i32,
    text: String,
    done: bool,
}
const PATH: &str = "~/.todo/todos.db";
fn main() -> Result<()> {
    let conn = Connection::open(PATH)?;
    conn.execute(
        "CREATE TABLE todo (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            done BOOLEAN NOT NULL,
        )",
        (), // empty list of parameters.
    )?;
    println!("Hello, world!");
    return Ok(())
}
