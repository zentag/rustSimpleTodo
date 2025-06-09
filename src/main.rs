use::std::io;
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Todo {
    id: i32,
    content: String,
    done: bool,
}
fn main() -> std::result::Result<(), rusqlite::Error> {
    println!("welcome to your todo list");
    let conn = Connection::open("./db.db3")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id   INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            done INTEGER
        )",
        (), // empty list of parameters.
    )?;
    let mut todo = String::new();
    io::stdin().read_line(&mut todo).expect("couldn't read stdin");
    println!("back to start");
    Ok(())
}
