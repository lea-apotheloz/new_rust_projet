use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::read_to_string;
use std::fs::{self};
use std::io;

#[derive(Serialize, Deserialize)]
struct Todo {
    message: String,
}

fn main() -> std::io::Result<()> {
    let mut todo = String::new();
    println!("write a to-do");
    io::stdin().read_line(&mut todo).expect("Read line failed.");

    let todo = todo.trim();

    //Open file.
    let mut todos: Vec<String> = match read_to_string("todo_list.json") {
        Err(_) => Vec::new(),
        Ok(todo_list) => todo_list.lines().map(String::from).collect(),
    };

    if todo.contains("--delete") {
        let test = todo.split(" ").last();
        let nomber_line: usize = test.expect("Error").parse().unwrap();

        todos.remove(nomber_line - 1);
    } else {
        // write to Vec
        todos.push(todo.to_string());
    }

    fs::write(
        "todo_list.json",
        serde_json::to_string(&todos).expect("Cannot serialize"),
    )
    .expect("Cannot write files");

    Ok(())
}
