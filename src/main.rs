use chrono::prelude::*;
use directories::UserDirs;
use std::fs;
use std::io;
use std::path::{ PathBuf };

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

fn create_folder(path: &std::path::Path) -> bool {
    if !path.exists() {
        if let Err(err) = fs::create_dir(path) {
            eprintln!("Failed to create folder: {}", err);
            false
        } else {
            true
        }
    } else {
        false
    }
}

fn create_file(path: &std::path::Path, base_filename: String) {
    let mut counter = 0;
    let new_filename = base_filename.to_string();

    loop {
        let file_path = path.join(format!("{}_{}.md", new_filename, counter));

        if !file_path.exists() {
            // Create the file
            if let Err(err) = fs::write(&file_path, "# My Markdown Content") {
                eprintln!("Failed to write file: {}", err);
            } else {
                println!("File '{}' created successfully.", file_path.to_string_lossy());
            }

            break; // Exit the loop after successfully creating the file
        } else {
            // File with the current name already exists, increment the counter
            counter += 1;
        }
    }
}

fn main() {
    match get_system_documents() {
        Ok(data_dir) => {
            let folder_name = "danielnotes";
            let mut danielnotes_path = data_dir.join(folder_name);
            create_folder(&danielnotes_path);

            println!("Which Company is this note for?");
            let mut company = String::new();
            io::stdin()
                .read_line(&mut company)
                .expect("Failed to read line");

            let company = company.trim();
            danielnotes_path.push(company);

            create_folder(&danielnotes_path);
            
            let text_file_string: String = {
                let local: DateTime<Local> = Local::now();
                let formatted_date = local.format("%Y-%m-%d").to_string();
                formatted_date
            };
            
            create_file(&danielnotes_path, text_file_string);


        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}
