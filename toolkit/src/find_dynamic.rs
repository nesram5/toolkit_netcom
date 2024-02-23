//find_dynamic.rs

use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;
use std::fs::OpenOptions;
use crate::ssh::{send_ssh_command_save_in_txt ,establish_ssh_connection};
use crate::auxiliar::{json_file_to_hash_string, return_path, clear_screen};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::process;
use ssh2::Channel;
use chrono::Local;

pub fn  option_4(username:String, password:String) -> io::Result<()> {
  
/*
    let ip_ftth_nodes: Vec<String> = vec![
        "10.1.51.1:22".to_string(),
        "10.1.20.1:22".to_string(),
        "10.10.44.1:22".to_string(),
        "10.1.8.1:22".to_string(),
        "10.10.48.1:22".to_string(),
        "10.10.56.1:22".to_string(),
        "10.10.36.1:22".to_string(),
        "10.1.50.1:22".to_string(),
        "10.0.3.2:22".to_string(),
        "10.1.44.1:22".to_string()];

    let ip_ffth_nodes_names: Vec<String> = vec![
        "FM".to_string(),
        "CEC".to_string(),
        "GU".to_string(), 
        "IL".to_string(), 
        "LP".to_string(), 
        "MS".to_string(), 
        "PLI".to_string(),
        "SLA".to_string(),
        "TE".to_string(), 
        "MRD".to_string()];


        // Serialize the vectors to JSON strings
    let ip_ftth_nodes_json = serde_json::to_string(&ip_ftth_nodes)?;
    let ip_ffth_nodes_names_json = serde_json::to_string(&ip_ffth_nodes_names)?;
    // Write JSON string to a file
    let mut _path = return_path();
    _path.push("data\\data.json");
    let mut file = File::create(_path.clone())?;
    writeln!(file, "{{\"ip_ftth_nodes\": {:?}, \"ip_ftth_nodes\": {:?}}}", ip_ftth_nodes, ip_ftth_nodes)?; */
    
    //ip_ftth_nodes ip_ffth_nodes_names


    /*
    Convert from JSON to VEC
     */
    
    let path_to_ftth_nodes = return_path("ftth_nodes.json", "data");
    
    let mut ftth_nodes:HashMap<String, String> = HashMap::new();
    match json_file_to_hash_string(path_to_ftth_nodes){
        Ok(res_ftth_nodes) => {
            ftth_nodes = res_ftth_nodes;
        },
        Err(e) => { eprintln!("No fue posible leer el archivo ftth_nodes.json  {}",e)}

    };
    //copy the keys in to a vec
    let name_of_the_nodes: Vec<String> = ftth_nodes.keys().cloned().collect();
    let _command = "ip dhcp-server lease print terse without-paging where dynamic".to_string();
   /* let mut file = File::open(path_ftth_nodes.clone())?;
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)?;

    // Parse JSON strings back into vectors
    let parsed: serde_json::Value = serde_json::from_str(&json_str)?;
    let ip_ftth_nodes = parsed["ip_ftth_nodes"].as_array().unwrap_or(&vec![]).iter().map(|v| v.as_str().unwrap_or("").to_string()).collect::<Vec<String>>();
    let ip_ffth_nodes_names = parsed["ip_ftth_nodes_names"].as_array().unwrap_or(&vec![]).iter().map(|v| v.as_str().unwrap_or("").to_string()).collect::<Vec<String>>();*/
    let current_date_to_file = Local::now().format("%d-%m-%Y").to_string();
    let current_time_to_file = Local::now().format("%I-%M-%p").to_string().to_lowercase();
    let output_file_name = format!("IP_Dinamicas_{}_{}.txt",current_date_to_file,current_time_to_file).to_string();

    let _path_dynamic_ips: std::path::PathBuf = return_path(&output_file_name, "cache");
    //let _path_string_dynamic_ips: String = _path_dynamic_ips.to_str().unwrap().to_string();
    //let copy_path = _path_string_dynamic_ips;
    //let filename = _path_dynamic_ips.to_str();
    let mut iterator = 0;
    let mut _address: String = String::new();
        for name_of_node in name_of_the_nodes{
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(_path_dynamic_ips.clone())?;
            _address = ftth_nodes.get(&name_of_node).unwrap().to_string();
            writeln!(file, "Ip dinamicas del nodo: {} {}\n",name_of_node,_address)?;
            

            let session = establish_ssh_connection(&_address, &username, &password)
            .map_err(|err| {
                io::Error::new(io::ErrorKind::Other, format!("No fue posible conectarse al router: {}         {}",_address, err))
            });
            
            let (sender, receiver) = mpsc::channel();
            // Spawn a new thread to execute the code
            let mut _channel_result: Option<Result<Channel, ssh2::Error>> = None;
            let _handle = thread::spawn(move || {

            let _channel = session.expect("Error").channel_session();

                sender.send(_channel).expect("Error de conexion");
            });

            let timeout_duration = Duration::from_secs(15);
            // Get the current time
            let _start_time = Instant::now();
            // Wait for the thread to finish or until the timeout is reached
            match receiver.recv_timeout(timeout_duration) {
                Ok(result) => {
                    // Update the channel_result variable with the received result
                    _channel_result = Some(result);                   
                }
                Err(_) => {
                    // Timeout reached 
                    clear_screen();
                    println!("\t\tNo fue posible conectarse al router {} \n",_address.clone());
                    process::exit(0);
                }
            }

            let looking_for = "address=";

                //Second timed command

            let (sender, receiver) = mpsc::channel();
                // Spawn a new thread to execute the code
            let _handle = thread::spawn(move || {
                let _path_dynamic_ips: std::path::PathBuf = return_path("dynamic_ips.txt", "cache");
                
                let command = "ip dhcp-server lease print terse without-paging where dynamic".to_string();
                let _line = send_ssh_command_save_in_txt(_channel_result.expect("error").unwrap(), command.clone().to_string(), looking_for, _path_dynamic_ips.clone());

                sender.send(_line).expect("Error de conexion");
            });

            let _timeout_duration = Duration::from_secs(15);
        // Get the current time
            let _start_time = Instant::now();
            
            match receiver.recv_timeout(_timeout_duration) {
                Ok(_line) => {
                }
                Err(_) => {
                    // Timeout reached            clear_screen();
                    println!("\t\tEl router {} se encuentra colgado",_address.clone());
                    writeln!(file, "El router {} se encuentra colgado\n",_address.clone())?;
                }
            }

            
            
            writeln!(file, "-----------------------------\n")?;
                

            iterator+=1;
            if iterator == name_of_node.len(){
                break;
            }
        }
    /*//Print Results
    let file = match File::open("dynamic_ips.txt") {
        Ok(f) => f,
        Err(e) => panic!("Error opening file: {}", e),
    };
    let reader = BufReader::new(file);
    clear_screen();
    println!("\t\tIp Dinamicas");
    let mut result:Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        result.push(line.trim().to_string());
        println!("{}",result.last().unwrap().as_str());
    }
        */
    let _ = Command::new("cmd.exe").arg("/c").arg("notepad.exe").arg(_path_dynamic_ips.clone()).status();
    println!("Abriendo el resultado en bloc de notas");
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();

    //let custom_error_message = "No fue posible conectarse al router";
    //io::Error::new(io::ErrorKind::Other, custom_error_message)
    Ok(())
}
