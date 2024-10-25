# user documentation

Creating a command-line todo list in Rust.
#### What's it for?

It's used to record information and modify it at any time.
#### To use it 

first, type install rust , then clone the project using the  `git clone*` command, then launch the terminal to save it using the `cd`command, followed by the path of the save. 
### Command:

 Put `cargo run` in front of each flag 
 - -- --delete --id (line number) 
- --done --id (line number) 
- --undone --id (line number) 
- --due YY-MM-DD --id (line number )
- -- --list 
- -- --sort

### explanation : 

 `--delete`deletes the line no longer required
 
 `--done`indicates that the todo-list is finished and ‘ undone ’ is the opposite: the todo is not finished.
 The `-- due` flag followed by the date in the correct format to set a deadline 

 setting the `--list` flag will display all the todo lists on the terminal in the order in which they were added, and they can be prioritized using the `--sort`flag.


### [video](./vidéo%20terminale%20.mp4)