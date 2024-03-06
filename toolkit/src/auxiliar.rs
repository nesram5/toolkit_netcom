//auxiliar.rs
use std::path::PathBuf;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process::Command;
use std::env;
use std::collections::HashMap;
use std::error::Error;
use serde_json::Value;

pub fn sort_ip_addresses(mut ip_addresses: Vec<String>) -> Vec<String> {
    ip_addresses.sort_by(|a, b| {
        let octets_a: Vec<u8> = a.split('.').flat_map(str::parse).collect();
        let octets_b: Vec<u8> = b.split('.').flat_map(str::parse).collect();

        for (octet_a, octet_b) in octets_a.iter().zip(octets_b.iter()) {
            if octet_a != octet_b {
                return octet_a.cmp(octet_b);
            }
        }

        std::cmp::Ordering::Equal
    });

    ip_addresses
}
/*
pub fn clean_txt_file(filename: &str) -> io::Result<()>{
    // Check if the file exists
    let file_exists = std::path::Path::new(filename).exists();

    // Open the file in append mode or create it if it doesn't exist
    let _file = if file_exists {
        File::create(filename)?
    } else {
        File::create(filename)?
    };
    Ok(())
}
pub fn save_vec_to_txt_file(data: Vec<String>, filename: &str) -> io::Result<()> {

    let mut file = OpenOptions::new().write(true).append(true).create(true).open(filename)?;
   
    // Iterate through the data and write each element to the file
    for line in data {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}*/

pub fn read_file_to_vec(filename: PathBuf) -> io::Result<Vec<String>> {
    // Open the file
    let file = File::open(filename)?;
       

    // Create a buffer reader to read the file line by line
    let reader = io::BufReader::new(file);

    // Create a vector to store lines
    let mut lines = Vec::new();

    // Read each line from the file and push it to the vector
    for line in reader.lines() {
        let l = line?;
        if !l.starts_with('#') {
            lines.push(l);
        }
    }

    // Return success and the vector of lines
    Ok(lines)
}

pub fn save_command_to_bat(data: Vec<String>, filename: PathBuf) -> io::Result<()> {

    let mut file = OpenOptions::new().write(true).create(true).open(filename)?;
   
    // Iterate through the data and write each element to the file
    for line in data {
        write!(file, "{}", line)?;
    }

    Ok(())
}

pub fn clear_screen() {
    if cfg!(unix) {
        // For Unix-like systems (Linux, macOS)
        Command::new("clear").status().expect("Failed to clear screen");
    } else if cfg!(windows) {
        // For Windows systems
        Command::new("cmd").arg("/c").arg("cls").status().expect("Failed to clear screen");
    } else {
        // Unsupported operating system
        println!("Clear screen not supported on this platform.");
    }
}

pub fn return_clean_vec_list_ip(filename: PathBuf)-> io::Result<Vec<String>>{
    // Open the JSON file
    let file = File::open(filename)?;
    // Read the JSON content into a string
    let mut flattened_data: Vec<String> = Vec::new();
    // Iterate over each line in the file
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let trimmed_line = line.trim();
        // Remove unwanted characters and split the line by commas
        let cleaned_line = trimmed_line
            .replace("{", "")
            .replace("}", "")
            .replace("[", "")
            .replace("]", "")
            .replace(",", "")
            .replace("=", "")
            //.replace(":", "")
            .replace("\"", " ")
            .replace("  ", "\n")
            .replace(" ", "")
            .replace(" ", "");
        // Push the cleaned line to the flattened data
        flattened_data.push(cleaned_line);
    }
    let filename = return_path("temp_list_ip", "cache");
    let mut file = OpenOptions::new().write(true).append(true).create(true).open(filename.clone())?;
       // Iterate through the data and write each element to the file
    for line in flattened_data {
        writeln!(file, "{}", line)?;
    }
    
    let file = File::open(filename.clone())?;
    // Create a buffer reader to read the file line by line
    let reader = io::BufReader::new(file);
    // Create a vector to store lines
    let mut lines = Vec::new();
    // Read each line from the file and push it to the vector
    for line in reader.lines() {
        let l = line?;
        if l.contains("\n"){
            continue;
        }
        else{
        lines.push(l);
        }
    }
    //delete temp file
    fs::remove_file(filename.clone())?;
    Ok(lines)
}
/*
pub fn vec_string_to_json(vec:Vec<String>, filename: PathBuf) -> std::io::Result<()>{
    let json_str = serde_json::to_string_pretty(&vec)?;

    // Write JSON string to a file
    let mut file = File::create(filename)?;
    file.write_all(json_str.as_bytes())?;
    Ok(())
}

pub fn read_json_to_vec(filename: PathBuf) -> Result<Vec<String>, io::Error> {
    // Open file
    let mut file = File::open(filename)?;

    // Read JSON string from file
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)?;

    // Deserialize JSON string to Vec<String>
    let strings: Vec<String> = serde_json::from_str(&json_str)?;

    Ok(strings)
}*/

pub fn return_path(filename: &str, data_or_cache: &str) -> PathBuf {
    let mut _path:PathBuf = env::current_dir().unwrap_or_default();
    let mut _join: PathBuf = PathBuf::new();
   if data_or_cache == "data"{
        _join = _path.join("data").join(filename);
    } else if data_or_cache == "cache" {
        _join = _path.join("cache").join(filename);
    }
    _join
}

pub fn print_first_five_lines(file_path: &str) -> io::Result<()> {
    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader to read the file line by line
    let reader = BufReader::new(file);

    // Use an iterator to iterate over the lines
    for (line_number, line) in reader.lines().enumerate() {
        // Check if we've printed 5 lines already
        if line_number >= 5 {
            break; // Exit the loop once we've printed 5 lines
        }

        // Print the line to the console
        println!("{}", line?);
    }

    Ok(())
}

pub fn json_file_to_hash_string(filename: PathBuf) -> Result<HashMap<String, String>, Box<dyn Error>> {
    // Open the JSON file
    let mut file = File::open(filename)?;

    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON string
    let parsed_json: Value = serde_json::from_str(&contents)?;

    // Convert the parsed JSON into a HashMap<String, String>
    let mut hash_string_map: HashMap<String, String> = HashMap::new();

    // Iterate over the JSON object
    if let Value::Object(map) = parsed_json {
        for (key, value) in map {
            if let Value::String(s) = value {
                hash_string_map.insert(key, s);
            }
        }
    }
    Ok(hash_string_map)
}

pub fn read_json_file_to_vec(filename: PathBuf) -> Result<Vec<(String, Vec<String>)>, Box<dyn Error>> {
    // Open the JSON file
    let mut file = File::open(filename)?;

    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON string into a serde_json::Value
    let parsed_json: Value = serde_json::from_str(&contents)?;

    // Convert the parsed JSON into a Vec<(String, Vec<String>)>
    if let Value::Object(map) = parsed_json {
        let vec_data: Vec<(String, Vec<String>)> = map
            .into_iter()
            .map(|(key, value)| {
                if let Value::Array(arr) = value {
                    let vec_value: Vec<String> = arr
                        .iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect();
                    (key, vec_value)
                } else {
                    (key, Vec::new())
                }
            })
            .collect();

        Ok(vec_data)
    } else {
        Err("JSON root is not an object".into())
    }
}

pub fn check_default_ro_to(login_or_segment:String) -> io::Result<String>{
    let filename = return_path("default_config.json", "data");
    let mut file = File::open(filename)?;

    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON string
    let json_value: Value = serde_json::from_str(&contents)?;

    // Extract the values from the JSON object
    let default_ro_test_login = json_value["default_ro_test_login"].as_str().unwrap_or_default().to_string();
    let default_ro_search_segments = json_value["default_ro_search_segments"].as_str().unwrap_or_default().to_string();
    
    if login_or_segment == "login"{
        return Ok(default_ro_test_login);
    } else if login_or_segment == "segment"{
        return Ok(default_ro_search_segments);
    }
    let string = String::new();
    Ok(string)
}