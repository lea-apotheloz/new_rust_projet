# technical documentation

#### first time

You need to install rust the link below will help you to install easily
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

Once rust is installed, click on the git repository link 
``` bash
git clone git@github.com:lea-apotheloz/new_rust_projet.git
```
The project must be found on the terminal
```bash
cd projet_rust_todo_list
```
### commentaire

 The file is in json format, which is lighter and more universal 
 there's plenty of documentation on how to learn it 


the code contains several structures to avoid repetition.


 There are a number of flags, so you can set them directly after cargo run.


 There's an id to replace the options at each flag.

 There are many libary:

 - serde
 - clap
 - chrono 

## code 
The first structure is used to announces everything we're going to write and use in the file and here types that will be used in the code,

 the second structure is for announcing all the flags we need. 

 Then checks all the flags, and if the user doesn't specify a flag, writes a new todo to the json file.
 


