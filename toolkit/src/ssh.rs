//ssh.rs
use ssh2::{Channel, Session};
use std::net::TcpStream;
use std::collections::HashSet;
use std::io::{self, BufRead, Read};
use std::env;
use std::path::PathBuf;
use crate::auxiliar::sort_ip_addresses;

pub fn establish_ssh_connection(_address: &String,username: &str, password: &str,) -> Result<Session, Box<dyn std::error::Error>> {
    let _tcp = TcpStream::connect(_address)?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(_tcp);
    sess.handshake()?;
    // Password-based authentication
    sess.userauth_password(username, password)?;
    if !sess.authenticated() {
        return Err("Authentication failed".into());
    }
    Ok(sess)
}

pub fn send_ssh_command_save_in_txt(mut channel: Channel , command: String, looking_for: &str, _filename: PathBuf) -> io::Result<Vec<String>> {
    
    let _channel: Result<(), ssh2::Error> = channel.exec(&command);
    
    // Read the output in chunks
    let mut buffer = Vec::new();
    let mut chunk = [0; 122880]; // Adjust the chunk size as needed
    let mut route_list:HashSet<String> = HashSet::new();
    
    loop {
        match channel.read(&mut chunk) {
            Ok(bytes_read) if bytes_read > 0 => {
                buffer.extend_from_slice(&chunk[..bytes_read]);

                // Check for your custom delimiter or process the data accordingly
                if let Some(delimiter_position) = buffer.iter().position(|&c| c == b'\n') {
                    // Process the data up to the delimiter (in this case, newline)
                    let output: Result<&str, std::str::Utf8Error> = std::str::from_utf8(&buffer[..delimiter_position]);
                    route_list.insert( format!("\n{:?}", output));

                    //REVOME THIS LATER
                    //println!("\n{:?}", output);
                     //REVOME THIS LATER

                    // Remove processed data from the buffer
                    buffer.drain(..delimiter_position + 1);
                }
            }
            Ok(_) => break,  // End of data
            Err(err) => return Err(err), // Handle errors
        }
    }
    //NOT SHOULD BE DATA REMAINING
    let remaining_output = std::str::from_utf8(&buffer).expect("Invalid UTF-8");

    // Create a BufReader around remaining_output
    let reader = io::BufReader::new(remaining_output.as_bytes());

    // Iterate over lines and insert each line into the HashSet
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        route_list.insert(line);
    }
    
    //Deleting duplicades
    let unique_list_of_ip: HashSet<String> = route_list
        .iter()
        .flat_map(|entry| entry.split(looking_for).nth(1))
        .flat_map(|gateway| gateway.split_whitespace().next())
        .filter(|&gateway| !gateway.is_empty()) // Filter out empty strings
        .map(String::from)
        .collect();

    let mut sorted_vec: Vec<String> = unique_list_of_ip.clone().into_iter().collect();
    sorted_vec = sort_ip_addresses(sorted_vec); // Sort the Vec

    //let _save_to_txt_file = save_vec_to_txt_file(sorted_vec , filename);
    Ok(sorted_vec)
}

pub fn command_ping_test_or_report(list_ip: Vec<String>, username: String, password: String , ping_or_report: usize) -> io::Result<Vec<String>> {
    let mut commands: Vec<String> = Vec::new();
    let api_path = {
        let mut path = env::current_dir().unwrap();
        path.push("api.exe");
        path
    };
    let api = api_path.to_str().unwrap();
    // Define the path to the WT executable file relative to the current directory
    let wt_patch: String  = api.to_string();
    let wt: String = wt_patch.replace("\\api.exe", "\\terminal\\wt.exe");
    //Check if ping or report mode 
    if ping_or_report == 1{
        let titletab = [
            "1-4",
            "5-8",
            "9-12",
            "13-16",
            "17-?",
        ];
        
        let mut n = 0;
        let mut i = 0;
        let j = list_ip.len() - 1;
        let echo_of ="ECHO OFF\n".to_string();
        commands.push(echo_of);
        while i < j {
            
            if i < j && i == 0 {
                
                let cmd1 = format!("\"{}\" -M -w 1 --title {} \"{}\" {} {} {} {} {} {} 1", 
                    wt, titletab[n], api, list_ip[i], list_ip[i+1], list_ip[i+2], list_ip[i+3], username, password);
                i += 4;
                commands.push(cmd1);
            } else if i < j {
                let cmd1 = format!("; -w 1 new-tab --title {} \"{}\" {} {} {} {} {} {} 1", 
                titletab[n], api, list_ip[i], list_ip[i+1], list_ip[i+2], list_ip[i+3], username, password);
                i += 4;
                commands.push(cmd1);
            } else {
                break;
            }

            if i < j {
                let cmd2 = format!("; -w 1 sp --title {} -V -c \"{}\" {} {} {} {} {} {} 1; mf right ",
                titletab[n], api, list_ip[i], list_ip[i+1], list_ip[i+2], list_ip[i+3], username, password);
                i += 4;
                commands.push(cmd2);
            } else {
                break;
            }

            if i < j {
                let cmd4 = format!("; -w 1 sp --title {} -H -c \"{}\" {} {} {} {} {} {} 1; mf left",
                titletab[n], api, list_ip[i], list_ip[i+1], list_ip[i+2], list_ip[i+3], username, password);
                i += 4;
                commands.push(cmd4);
            } else {
                break;
            }

            if i < j {
                let cmd6 = format!("; -w 1 sp --title {} -H -c \"{}\" {} {} {} {} {} {} 1",
                titletab[n], api, list_ip[i], list_ip[i+1], list_ip[i+2], list_ip[i+3], username, password);
                i += 4;
                n += 1;
                commands.push(cmd6);
            } else {
                break;
            }
        }
    } else if ping_or_report == 2{
        //report mode
                
        let mut i = 0;
        let j = list_ip.len() - 1;
        let _echo_off ="ECHO OFF\n".to_string();
        commands.push(_echo_off);
        while i < j {
            
            if i < j && i == 0 {
                
                let cmd1 = format!("START /B \"\" \"{}\" {} {} {} {} {} {} 2\n", 
                    api, list_ip[i], list_ip[i+1], list_ip[i+2], list_ip[i+3], username, password);
                i += 4;
                commands.push(cmd1);
            } else if i < j {
                let cmd1 = format!("START /B \"\" \"{}\" {} {} {} {} {} {} 2\n",  
                api, list_ip[i], list_ip[i+1], list_ip[i+2], list_ip[i+3], username, password);
                i += 4;
                commands.push(cmd1);
            } else {
                break;
            }

            if i < j {
                let cmd2 = format!("START /B \"\" \"{}\" {} {} {} {} {} {} 2\n",  
                api, list_ip[i], list_ip[i+1], list_ip[i+2], list_ip[i+3], username, password);
                i += 4;
                commands.push(cmd2);
            } else {
                break;
            }

            if i < j {
                let cmd4 = format!("START /B \"\" \"{}\" {} {} {} {} {} {} 2\n",  
                api, list_ip[i], list_ip[i+1], list_ip[i+2], list_ip[i+3], username, password);
                i += 4;
                commands.push(cmd4);
            } else {
                break;
            }

            if i < j {
                let cmd6 = format!("START /B \"\" \"{}\" {} {} {} {} {} {} 2\n",
                api, list_ip[i], list_ip[i+1], list_ip[i+2], list_ip[i+3], username, password);
                i += 4;
                commands.push(cmd6);
            } else {
                break;
            }
        }
    }
    

    Ok(commands)
}