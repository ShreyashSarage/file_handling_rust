use std::fs;
use std::env;
use serde_json::Value;
fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    // Read the contents of the json file
    let config_contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading config.json: {}", e);
            return;
        }
    };

    // Parse the JSON contents
    let config_json: Value = match serde_json::from_str(&config_contents) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error parsing config.json: {}", e);
            return;
        }
    };

    // Extract the file_path from the JSON
    let file_path = match config_json["file_path"].as_str() {
        Some(path) => path,
        None => {
            eprintln!("No file_path specified in config.json");
            return;
        }
    };

    // println!("Searching for {}", query);
    println!("File path {}", file_path);


    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading {}: {}", file_path, e);
            return;
        }
    };

    println!("With text:\n{contents}");
}
