use std::io::{self,Write};
use std::fs::OpenOptions;
fn main() -> io::Result<()> {
    println!("write a to-do");
    let mut todo= String::new();
    io::stdin().read_line(&mut todo)?;

    //open the new file in append mode
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("todo.txt")?;
    
    // Remove white spaces at the beginning and end
    let todo = todo.trim();

    // write to text file
    writeln!(file, "{}", todo)?;
    Ok(())

}
