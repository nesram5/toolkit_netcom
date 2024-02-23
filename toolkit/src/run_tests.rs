//run_tests.rs

use std::process::Command;
use std::fs;
use crate::auxiliar::save_command_to_bat;
use crate::ssh::command_ping_test_or_report;
use crate::auxiliar::return_path;



pub fn option_2(list_ip: Vec<String>, username: String, password: String){
    
    let path_commands = return_path("commands.bat", "cache");
    println!("Ejecutando pruebas de ping en otra ventana");
    let _commands: Vec<String> = Vec::new();
    match command_ping_test_or_report(list_ip, username, password, 1){
        Ok(_commands) => {
            let _save_to_txt_file =  save_command_to_bat(_commands, path_commands.clone());
          
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
    ;
    let command_ = path_commands.to_str().unwrap_or_default();
        let _status = Command::new("cmd.exe")
        .arg("/c")
        .arg(command_)
        .status()
        .expect("Failed to execute bat file");
        let _file_path = "commands.bat"; 
        match fs::remove_file(path_commands.clone()) {
          Ok(_) => println!("File '{}' deleted successfully", _file_path),
           Err(err) => eprintln!("Failed to delete file '{}': {}", _file_path, err),
        }
        
}
