use mysql_async::Conn;
use std::io::{self, BufRead, Cursor, BufReader};
use ssh2::Channel;
use std::process::Command;
use std::str;
use std::error::Error;
use std::io::Read;
use crossterm::{execute, cursor};
const BANNER_LINE: u16 = 20;
const PING_RESULTS_START_LINE: u16 = 1;
const PING_RESULTS_END_LINE: u16 = 16;

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::process;

use crate::ssh::establish_ssh_connection;
use crate::auxiliar::{calculate_packet_loss, calculate_average_latency, parse_latency_value, calculate_average_ttl, clear_screen, print_line, clear_lines, find_min_max};
use crate::upload_to_db::{upload_to_db, connect_to_db};

pub fn upload_mode(username: &str, password: &str, address: &str, title: &str, command: &str)  ->  std::io::Error {
    //ON live logic
   let _address = String::from(address);
   let mut i = 0;
   loop {
    
    let _session = establish_ssh_connection(address.to_string(), &username, &password)
        .map_err(|_err| {
            // Wrap the error in a custom std::io::Error
            io::Error::new(io::ErrorKind::Other, format!("No fue posible establecer sesion con el router: {} ",&address))
        });

    let (sender, receiver) = mpsc::channel();

    // Spawn a new thread to execute the code
    let mut _channel_result: Option<Result<Channel, ssh2::Error>> = None;

    // Spawn a new thread to execute the code
    let _handle = thread::spawn(move || {
        // Place your code here that you want to execute
        // For demonstration, we'll just print some messages
        let _channel: Result<Channel, ssh2::Error> = _session.expect("No fue posible establecer el canal").channel_session();

        // Send the result through the channel when the task is done
        sender.send(_channel).expect("Error de conexion");
    });

    // Set the timeout duration
    let timeout_duration = Duration::from_secs(30);

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
            println!("\t\tNo fue posible conectarse al router {} \n\t\t{}",&address, &title);
            let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
            //process::exit(0);
        }
    }

      //Time controled command 2
      let (sender, receiver) = mpsc::channel();
      let _address = address.to_string();
      let _title = title.to_string();
      let _command = command.to_string();      
      let _handle = thread::spawn(move || {
         let res = ping_test_continous_output(_channel_result.expect("ss").unwrap(), _title, _address, _command).unwrap();
 
         sender.send(res).expect("Error de conexion");
     });
 
     let _timeout_duration = Duration::from_secs(30);
         // Get the current time
     let _start_time = Instant::now();
     
     match receiver.recv_timeout(_timeout_duration) {
         Ok(_res) => {
             println!("");
         }
         Err(_) => {
             // Timeout reached 
             clear_screen();
             println!("\t\tEl router {} se encuentra colgado",&address);
             let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
             //process::exit(0);
         }
     }
     
     i+=1;
     if i == 3{
        break;
     }     
     println!("Vamos a intentarlo de nuevo");
     std::thread::sleep(Duration::from_secs(2));
    }

    let custom_error_message = "No fue posible conectarse al router";
    io::Error::new(io::ErrorKind::Other, custom_error_message)
}


fn process_ssh_terminal(buffer: &mut [u8; 4096],address:String, title: String) -> (Vec<f32>){
    let mut ttl: Vec<i32> = Vec::new();
    let mut latency: Vec<f32> = Vec::new();
    let mut reader = BufReader::new(Cursor::new(&mut buffer[..]));

    
    loop {
        let mut line = String::new();
        if let Ok(bytes_read) = reader.read_line(&mut line) {
            if bytes_read == 0 {
                break;
            }
        } else {
            eprintln!("Error reading from stdout");
            break;
        }       

        // Header line
        if line.contains("SEQ HOST") {
            continue;
        }
        // Averages line
        else if line.contains("avg-rtt=") && line.contains("packet-loss=") {
            continue;
        }
        // Ping value line

        else if line.trim().ends_with(&['m', 's', 'u', 's'][..]) {
            
            let combined_value = parse_latency_value(&line);
            latency.push(combined_value);
 
            // Grab TTL value
            if line.len() >= 5 {
                ttl.push(line.split_whitespace().nth(3).unwrap_or("0").parse().unwrap_or(0));
            }
        }
 
        // Handle errors
        else if line.contains("could not...") || line.contains("packet-loss=100%") || line.contains("timeout") {
            latency.push(9999.0);
            println!("{}", line);

        }
        else if line.contains("Invalid ar...") {
            println!("\t\tComando invalido, verifica si la interfaz o la ip esta en uso en el router {} \n\t\tTest nombre: {}",address.clone(), title);
            let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
            process::exit(0);
            
        }
        //Deleted because cause ramdom Packet Loss results
        // Unknown line
        //else {
        //    //println!("Unknown Line:");
        //    latency.push(9999.0);
        //    println!("{}", line);
        //}
 
        
        if latency.len() != 0 {
            return (latency);
        }
    }
    (latency)

}
 
async fn ping_test_continous_output(mut channel: Channel, title: String, address: String, command: String) -> Result<(), Box<dyn Error>> {
    clear_screen();
    let mut latency: Vec<f32> = Vec::new();

    // Open a channel and execute the command
    channel.exec(&command)?;

    // Connect to the database
    let mut db_connection = connect_to_db().await?;

    let mut buffer = [0; 4096];
    let mut line_number = 1;

    loop {
        let len = channel.read(&mut buffer)?;
        if len == 0 {
            // End of output
            break;
        }

        // Process the continuous output
        let latency_result = process_ssh_terminal(&mut buffer, address.to_string(), title.to_string());
        latency.extend(latency_result);

        // Upload latency data to the database
        upload_to_db(&mut db_connection, &latency).await?;

        if line_number < PING_RESULTS_END_LINE {
            print_line(&buffer_str, line_number)?;
            line_number += 1;
        } else {
            line_number = 0;
            clear_lines(PING_RESULTS_START_LINE, PING_RESULTS_END_LINE + 3)?;
        }

        if latency.len() == 10 {
            clear_screen();
        }

        // Clean buffer
        buffer = [0; 4096];

        // Clean latency
        if latency.len() == 100 {
            latency.remove(0);
        }
    }

    Ok(())
}