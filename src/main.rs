use chrono::prelude::*;
use directories::UserDirs;
use std::fs;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

static mut DOWNLOADS_PATH: Option<PathBuf> = None;
static mut DOCUMENTS_FOLDER: Option<PathBuf> = None;


fn get_current_date() -> String {
    // Get the current date and time in the local timezone
    let local: DateTime<Local> = Local::now();

    // Format the date as a string
    let formatted_date = local.format("%Y-%m-%d").to_string();

    // Return the formatted date string
    formatted_date
}

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

fn copy_file(file: &String, destination: &String) -> std::io::Result<()> {
    // Access DOWNLOADS_PATH in unsafe block
    unsafe {
        if let Some(downloads_path) = &DOWNLOADS_PATH {
            // Clone the path so we can append the file name
            let mut path = downloads_path.clone();
            // Append the file name to the path
            path.push(file);

            // Check if the file exists
            if path.exists() {
                println!("File {} exists in downloads directory.", file);
            } else {
                println!("Could not find file {} in downloads directory", file);
            }
        }
    }

    Ok(())
}

fn create_file(path: &Path, base_filename: String) {
    let current_date = get_current_date();

    let mut counter = 0;

    // Construct the file path with base_filename, notes, and date
    let mut file_path = path.join(format!("{}/notes/{}.md", base_filename, current_date));

    // Ensure the directory structure exists
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            if let Err(err) = fs::create_dir_all(parent) {
                eprintln!("Failed to create directory: {}", err);
                return; // Return early if directory creation fails
            }
        }
    }
    loop {
        if !file_path.exists() {
            // Create the file
            if let Err(err) = fs::write(&file_path, "# My Markdown Content") {
                eprintln!("Failed to write file: {}", err);
            } else {
            }

            break; // Exit the loop after successfully creating the file
        } else {
            // Increment the counter and update the file name
            counter += 1;
            file_path = path.join(format!(
                "{}/notes/{}_{}.md",
                base_filename, current_date, counter
            ));
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
                Err("Invalid number of inputs for 'file'. Please enter exactly three.")
            }
        }
        "fetch" => {
            if input_parts.len() == 3 {
                Ok(input_parts)
            } else {
                Err("Invalid number of inputs for 'file'. Please enter exactly three.")
            }
        }
        _ => {
            // Return input_parts for other cases
            Ok(input_parts)
        }
    }
}

fn fetch_files(path: &Path) -> io::Result<Vec<String>> {
    let mut file_names = Vec::new();

    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();

        let file_name_str = file_name.to_string_lossy().into_owned();

        if !file_name_str.starts_with('.') {
            file_names.push(file_name_str.clone());
        }
    }

    Ok(file_names)
}

fn main() {
    match get_system_documents() {
        Ok(data_dir) => {
            let folder_name = "danielnotes";
            unsafe {
                DOCUMENTS_FOLDER = Some(data_dir.join(folder_name));
            }
            if let Some(path) = unsafe { DOCUMENTS_FOLDER.as_ref() } {
                create_folder(path);
                println!("Welcome to danielnotes!");
            } else {
                eprintln!("Error: DOCUMENTS_FOLDER is not available.");
            }
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }

    match get_downloads_folder() {
        Ok(data_dir) => {
            unsafe {
                DOWNLOADS_PATH = Some(data_dir);
            }
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }

    loop {
        match parse_command() {
            Ok(parsed_input) => {
                // Handle the parsed_input based on the command
                match parsed_input[0].as_str() {
                    "create" => {
                        if let Some(path) = unsafe { DOCUMENTS_FOLDER.as_ref() } {
                            create_file(path.as_ref(), parsed_input[1].clone());
                        } else {
                            eprintln!("Error: DOCUMENTS_FOLDER is not available.");
                        }
                    }
                    "file" => {
                        copy_file(&parsed_input[1], &parsed_input[2]);
                    }
                    "fetch" => {
                        println!(
                            "Handling 'fetch' case with parameters: {}, {}",
                            parsed_input[1], parsed_input[2]
                        );
                    }
                    "list" => {
                        if let Some(path) = unsafe { DOCUMENTS_FOLDER.as_ref() } {
                            match fetch_files(path) {
                                Ok(files) => {
                                    println!("Companies in danielnotes: {}", files.join(","));
                                }
                                Err(_) => println!("Error fetching files."),
                            }
                        }
                    }
                    _ => {
                        println!("Unknown command. Exiting");
                        break;
                    }
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
}