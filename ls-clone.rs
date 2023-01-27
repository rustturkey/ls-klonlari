use std::fs;
use std::io;

fn main() {
    let paths = fs::read_dir("./").unwrap();
    
    let mut user_input = String::new();

    let stdin = io::stdin();

    stdin.read_line(&mut user_input);

    //if user_input == "ls" does not work
    if user_input.trim() == "ls"
    {
    
        for path in paths {
            println!("{}", path.unwrap().path().display())
        }
    }

    else
    {}
}