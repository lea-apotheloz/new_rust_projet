use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::read_to_string;
use std::fs::{self};
use std::io;
use clap::Parser;


#[derive(Serialize, Deserialize)]
struct Todo {
    message: String,
}
#[derive(Parser)]

struct Flag{
    #[arg(long,short)]
    delete: Option<usize>,

}

fn main() -> std::io::Result<()> {
    let flag= Flag::parse();

    //Open file.
    let mut todos: Vec<Todo> = match read_to_string("todo_list.json") {
        Err(_) => Vec::new(),
        Ok(todo_list) => serde_json::from_str(&todo_list).expect("Err"),
    };

    

if let Some(number_line)= flag.delete {
    if number_line > 0 && number_line <= todos.len(){
        todos.remove(number_line -1);
    };

}else {
    let mut todo = String::new();
    println!("write a to-do");
    io::stdin().read_line(&mut todo).expect("Read line failed.");

    let todo = todo.trim();

    if !todo.is_empty(){
        todos.push(Todo{message: todo.to_string()});
    }
}


  //  if todo.contains("--delete") {
    //    let test = todo.split(" ").last();
      //  let nomber_line: usize = test.expect("Error").parse().unwrap();

     //   todos.remove(nomber_line - 1);
   // } else {
        // write to Vec
        
    

    fs::write(
        "todo_list.json",
        serde_json::to_string(&todos).expect("Cannot serialize"),
    )
    .expect("Cannot write files");

    Ok(())
}

