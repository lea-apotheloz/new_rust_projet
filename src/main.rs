use chrono::NaiveDate;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs::read_to_string;
use std::fs::{self};
use std::io;

/// implementation of a struct to be able to think of text format in json format
#[derive(Serialize, Deserialize)]
struct Todo {
    message: String,
    status: bool,
    deadline: Option<NaiveDate>,
}

///structure allows that on the terminal directly type the flag searched by the user
#[derive(Parser)]
struct Flag {
    /// write number line
    /// delete the line you no longer want
    #[arg(long, short)]
    delete: bool,

    /// write number line
    /// indicate that the todo is finished
    #[arg(long)]
    done: bool,

    ///write number line
    /// indicate that the dodo is not finished
    #[arg(long)]
    undone: bool,

    /// write a date
    /// add a dealine 
    #[arg(long)]
    due: Option<String>,

    ///display todos ans status
    #[arg(long)]
    list: bool,

    #[arg(long)]
    sort: bool,

    #[arg(long, default_value_t = 0)]
    id: usize,
}

fn main() -> std::io::Result<()> {
    let flag = Flag::parse();

    //Open file.
    let mut todos: Vec<Todo> = match read_to_string("todo_list.json") {
        Err(_) => Vec::new(),
        Ok(todo_list) => serde_json::from_str(&todo_list).expect("Err"),
    };

    if flag.delete {
        todos.remove(flag.id - 1);
    } else if flag.done {
        todos[flag.id - 1].status = true;
    } else if flag.undone {
        todos[flag.id - 1].status = false;
    } else if let Some(due_date) = flag.due {
        match NaiveDate::parse_from_str(&due_date, "%Y-%m-%d") {
            Ok(date) => {
                todos[flag.id - 1].deadline = Some(date);
            }

            Err(_) => {
                println!("invalid date format!")
            }
        }
    } else if flag.list {
        for (_i, todo_list) in todos.iter().enumerate() {
            let _status = if todo_list.status { "done" } else { "undone" };
            println!("{}. {} =  {}", _i + 1, todo_list.message, _status);
        }
    } else if flag.sort {
        // Using sort_by
        todos.sort_by(|a, b| a.deadline.cmp(&b.deadline));
        
    }
    
    
    
    else {
        let mut todo = String::new();
        println!("write a to-do");
        io::stdin().read_line(&mut todo).expect("Read line failed.");

        let todo = todo.trim();

        let user_todo = Todo {
            message: todo.to_string(),
            status: false,
            deadline: None,
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
