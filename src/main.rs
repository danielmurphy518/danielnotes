//use directories::ProjectDirs;
//use std::fs;
//use std::path::Path;
use directories::UserDirs;
use std::io;


fn get_system_documents() -> Result<std::path::PathBuf, &'static str> {
    if let Some(user_dirs) = UserDirs::new() {
        if let Some(data_dir) = user_dirs.document_dir() {
            return Ok(data_dir.to_path_buf());
        } else {
            return Err("Data Directory not available.");
        }
    }
    Err("UserDirs not available.")
}

fn main() {
    match get_system_documents() {
        Ok(data_dir) => {
            println!("Data: {:?}", data_dir);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }

    println!("Enter something:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("You entered: {}", input.trim());
}