use std::fs;
use std::path::Path;
use std::env;
use chrono::{DateTime, Local};

fn main() {
    let path = Path::new("."); // current directory
    let current_dir = env::current_dir().unwrap();
    
    
    //Current directory
    println!("Current directory: {:#?}", current_dir); 

    
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name(); 
        let file_name = file_name.to_str().unwrap(); 
  
        let metadata = entry.metadata().unwrap();
        let size = metadata.len();
        
        let modified: DateTime<Local> = DateTime::from(metadata.modified().unwrap());

        

        println!("
        LastWriteTime         Length       Name
        -------------         ------       ----
            {:<5}       {:>5}      {:>5}
    
        ",modified.format("%_d %b %H:%M").to_string(),size,
        file_name);
           
        
        
        
    }

    
}




