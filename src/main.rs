use::std::io;
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Todo {
    id: i32,
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
    for todo in todo_iter {
        let todo = todo.unwrap();
        let x_or_nothing;
        if todo.done == true {
            x_or_nothing = "x";
        } else {
            x_or_nothing = " ";
        }
        println!("- [{x_or_nothing}] {}", todo.content);
    }
    println!("enter command...");
    let mut cmd = String::new();
    io::stdin().read_line(&mut cmd).expect("couldn't read stdin");
    match &cmd[..2] {
        "t " => {
            let position: &i32 = &cmd[2..].trim().parse().expect("Not a valid number");
println!("{position}");
        },
        "d " => println!("deleting"),
        _ => println!("creating")
    } 
    Ok(())
}
