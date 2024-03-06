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
  
    let path_to_ftth_nodes = return_path("ftth_nodes.json", "data");
    
    let mut ftth_nodes:HashMap<String, String> = HashMap::new();
    match json_file_to_hash_string(path_to_ftth_nodes){
        Ok(res_ftth_nodes) => {
            ftth_nodes = res_ftth_nodes;
        },
        Err(e) => { eprintln!("No fue posible leer el archivo ftth_nodes.json  {}",e)}

    };
    let name_of_the_nodes: Vec<String> = ftth_nodes.keys().cloned().collect();
    let _command = "ip dhcp-server lease print terse without-paging where dynamic".to_string();
   
    let current_date_to_file = Local::now().format("%d-%m-%Y").to_string();
    let current_time_to_file = Local::now().format("%I-%M-%p").to_string().to_lowercase();
    let output_file_name = format!("IP_Dinamicas_{}_{}.txt",current_date_to_file,current_time_to_file).to_string();

    let _path_dynamic_ips: std::path::PathBuf = return_path(&output_file_name, "cache");
    
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

            let (sender2, receiver2) = mpsc::channel();
                // Spawn a new thread to execute the code
            let _handle = thread::spawn(move || {
                let _path_dynamic_ips: std::path::PathBuf = return_path("dynamic_ips.txt", "cache");
                
                let command = "ip dhcp-server lease print terse without-paging where dynamic".to_string();
                let _line = send_ssh_command_save_in_txt(_channel_result.expect("error").unwrap(), command.clone().to_string(), looking_for, _path_dynamic_ips.clone());

                sender2.send(_line).expect("Error de conexion");
            });

            let _timeout_duration2 = Duration::from_secs(15);
        // Get the current time
            let _start_time2 = Instant::now();
            
            match receiver2.recv_timeout(_timeout_duration2) {
                Ok(_line) => {
                    for line in _line?{
                        writeln!(file,"{}", line)?;
                    }
                    
                }
                Err(_) => {
                    // Timeout reached            clear_screen();
                    println!("\t\tEl router {} se encuentra colgado",_address.clone());
                    writeln!(file, "El router {} se encuentra colgado\n",_address.clone())?;                    
                }
            }
            writeln!(file, "-----------------------------\n")?;
        }

    let _ = Command::new("cmd.exe").arg("/c").arg("notepad.exe").arg(_path_dynamic_ips.clone()).status();
    println!("Abriendo el resultado en bloc de notas");
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();

    Ok(())
}
