use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs::read_to_string;
use std::fs::{self};
use std::io;
use clap::Parser;

/// implementation of a struct to be able to think of text format in json format
#[derive(Serialize, Deserialize)]
struct Todo {
    message: String,
    status: bool,

    
}

///structure allows that on the terminal directly type the flag searched by the user
#[derive(Parser)]
struct Flag{
    
    /// write number line 
    /// delete the line you no longer want
    #[arg(long,short, default_value_t = 0)]
    delete: usize,

    /// write number line 
    /// indicate that the todo is finished
    #[arg(long, default_value_t= 0)]
    done: usize,

    ///write number line 
    /// indicate that the dodo is not finished
    #[arg(long,default_value_t = 0)]
    undone: usize,
}

fn main() -> std::io::Result<()> {
    let flag= Flag::parse();

    //Open file.
    let mut todos: Vec<Todo> = match read_to_string("todo_list.json") {
        Err(_) => Vec::new(),
        Ok(todo_list) => serde_json::from_str(&todo_list).expect("Err"),
    };

    

if flag.delete > 0 && flag.delete <= todos.len(){
        todos.remove(flag.delete -1);

    }else if flag.done > 0&& flag.done <= todos.len() {
        todos[flag.done -1].status = true;

    }else if flag.undone > 0 && flag.undone <= todos.len(){
        todos[flag.done -1].status = false;
    }
    
    else{
    let mut todo = String::new();
    println!("write a to-do");
    io::stdin().read_line(&mut todo).expect("Read line failed.");


    let todo = todo.trim();

    let user_todo = Todo{
        message: todo.to_string(),
        status:false,
        
    };
   
     todos.push(user_todo);
     

    
}
        
    fs::write(
        "todo_list.json",
        serde_json::to_string(&todos).expect("Cannot serialize"),
    )
    .expect("Cannot write files");

    Ok(())
}

