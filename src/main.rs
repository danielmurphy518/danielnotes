use directories::UserDirs;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn get_system_documents() -> Result<PathBuf, &'static str> {
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

            // Create a folder called "danielnotes" in the documents directory
            let folder_name = "danielnotes";
            let danielnotes_path = data_dir.join(folder_name);

            if !danielnotes_path.exists() {
                if let Err(err) = fs::create_dir(&danielnotes_path) {
                    eprintln!("Failed to create folder: {}", err);
                    return;
                }
            }

            // Read input from the user to create a subfolder
            println!("Enter a folder name:");
            let mut folder_input = String::new();
            io::stdin().read_line(&mut folder_input).expect("Failed to read line");

            // Create a folder with the user's input under the "danielnotes" folder
            let folder_path = danielnotes_path.join(folder_input.trim());
            if !folder_path.exists() {
                if let Err(err) = fs::create_dir(&folder_path) {
                    eprintln!("Failed to create folder: {}", err);
                    return;
                }
            }

            // Read additional input from the user and write it to a file under the created folder
            println!("Enter something to write to a file:");
            let mut file_input = String::new();
            io::stdin().read_line(&mut file_input).expect("Failed to read line");

            // Create a file with the user's input under the created folder
            let file_path = folder_path.join("user_input.txt");
            if let Err(err) = fs::write(&file_path, file_input.trim()) {
                eprintln!("Failed to write file: {}", err);
            } else {
                println!("File '{}' created successfully.", file_path.display());
            }
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}