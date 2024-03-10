use chrono::prelude::*;
use directories::UserDirs;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::io::Write;


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

fn get_downloads_folder() -> Result<PathBuf, &'static str> {
    if let Some(user_dirs) = UserDirs::new() {
        if let Some(data_dir) = user_dirs.download_dir() {
            return Ok(data_dir.to_path_buf());
        } else {
            return Err("Data Directory not available.");
        }
    }
    Err("DownloadDirs not available.")
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

fn copy_file(source_path: &std::path::Path, destination_path: &std::path::Path) -> io::Result<()> {
    // Attempt to copy the file
    fs::copy(source_path, destination_path)?;

    println!("File copied successfully!");

    Ok(())
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

fn parse_command() -> Result<Vec<String>, &'static str> {
    let mut input = String::new();
    println!("Please enter a command!");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input_parts: Vec<String> = input.trim().split_whitespace().map(String::from).collect();

    if input_parts.is_empty() {
        return Err("No command entered. Please provide a command.");
    }

    match input_parts[0].as_str() {
        "create" => {
            if input_parts.len() == 2 {
                Ok(input_parts)
            } else {
                Err("Invalid number of inputs for 'create'. Please enter exactly two.")
            }
        }
        "file" => {
            if input_parts.len() == 3 {
                Ok(input_parts)
            } else {
                Err("Invalid number of inputs for 'file'. Please enter exactly four.")
            }
        }
        _ => {
            Err("Unsupported command. Please enter a valid command.")
        }
    }
}

fn main() {
    match parse_command() {
        Ok(parsed_input) => {
            // Handle the parsed_input based on the command
            match parsed_input[0].as_str() {
                "create" => {
                    // Handle 'create' case with parsed_input[1]
                    println!("Handling 'create' case with parameter: {}", parsed_input[1]);
                }
                "file" => {
                    // Handle 'file' case with parsed_input[1], parsed_input[2], and parsed_input[3]
                    println!("Handling 'file' case with parameters: {}, {}", parsed_input[1], parsed_input[2]);
                }
                _ => {
                    println!("Unknown command. No specific action taken.");
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}