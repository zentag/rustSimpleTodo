use std::io;
use rusqlite::{Connection, Result};
use rand::Rng;

#[derive(Debug)]
struct Todo {
    id: u32,
    content: String,
    done: bool,
}
fn main() -> Result<()> {
    println!("welcome to your todo list");
    println!("controls: enter todo content to make new todo, use t <position in list> to toggle todo, and use d <position> to delete todo");
    let conn = Connection::open("./db.db3")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id   INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            done INTEGER
        );",
        (), // empty list of parameters.
    )?;

    let mut stmt = conn.prepare("SELECT id, content, done FROM todos")?;
    let todo_iter = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            content: row.get(1)?,
            done: row.get(2)?,
        })
    })?;
    let mut todos = vec![];
    for todo in todo_iter {
        let todo = todo.unwrap();
        let x_or_nothing;
        if todo.done == true {
            x_or_nothing = "x";
        } else {
            x_or_nothing = " ";
        }
        println!("- [{x_or_nothing}] {}", todo.content);
        todos.push(todo);
    }
    println!("enter command...");
    let mut cmd = String::new();
    io::stdin().read_line(&mut cmd).expect("couldn't read stdin");
    match &cmd[..2] {
        "t " => {
            let position: usize = cmd[2..].trim().parse().expect("Not a valid number");
            let todo = &todos[position];
            let id = &todo.id;
            let done_int;
            if &todo.done == &true {
                done_int = 0;
            } else {
                done_int = 1;
            }
            conn.execute(&format!("UPDATE todos SET done = {done_int} WHERE id = {id};"), ())?;
        },
        "d " => {
            let position: usize = cmd[2..].trim().parse().expect("Not a valid number");
            let todo = &todos[position];
            let id = &todo.id;
            conn.execute(&format!("DELETE FROM todos WHERE id={id}"),())?; 
        },
        _ => {
            let id = rand::rng().random_range(0..2^32);
            let trimmed_cmd = cmd.trim();
            conn.execute(&format!("INSERT INTO todos (id, content, done) VALUES ({id}, '{trimmed_cmd}', 0)"),())?;
        }
    } 
    Ok(())
}
