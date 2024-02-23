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


pub fn ping_test_mode(username: &str, password: &str, address:String, title:String, command: String)  ->  std::io::Error {
    //ON live logic
    let _address = address.clone();

    let _session = establish_ssh_connection(&address, &username, &password)
        .map_err(|_err| {
            // Wrap the error in a custom std::io::Error
            io::Error::new(io::ErrorKind::Other, format!("No fue posible establecer sesion con el router: {} ",address))
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
            clear_screen();
            println!("\t\tNo fue posible conectarse al router {} \n\t\t{}",address.clone(), title);
            let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
            process::exit(0);
        }
    }

      //Time controled command 2
      let (sender, receiver) = mpsc::channel();
            
      let _handle = thread::spawn(move || {
         let res = ping_test_continous_output(_channel_result.expect("ss").unwrap(),  &title, address.clone(), &command).unwrap();
 
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
             println!("\t\tEl router {} se encuentra colgado",_address.clone());
             let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
             process::exit(0);
         }
     }


    let custom_error_message = "No fue posible conectarse al router";
    io::Error::new(io::ErrorKind::Other, custom_error_message)
}


fn process_ssh_terminal(buffer: &mut [u8; 4096],address:String, title: &String) -> (Vec<f32>, Vec<i32>){
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
            return (latency , ttl);
        }
    }
    (latency, ttl)

}
 
fn ping_test_continous_output (mut channel: Channel, title: &String, address:String, command: &String) -> Result<(), Box<dyn Error>> {
    clear_screen();
    let mut latency:Vec<f32> = Vec::new();
    let mut ttl:Vec<i32> = Vec::new();
    let mut _latency_average:f32 = 0.0;
    let mut _packet_loss:f32 = 0.0;
    let mut _min_value:f32 = 0.0;
    let mut _max_value:f32 = 0.0;
    let mut _ttl_average:i32 = 0;
        // Open a channel and execute the command0480

    let _channel: Result<(), ssh2::Error> = channel.exec(command);    


    let mut buffer = [0; 4096];
    let mut line_number = 1;
    //let mut iteration = 0;
    //let mut iteration_clean_screen = 0;
    clear_screen();

    Ok(loop {
        let len = channel.read(&mut buffer)?;
        if len == 0 {
            // End of output
            break;
        }
        
        // Process the continuous output
        //print!("{}", str::from_utf8(&buffer)?);
        let (latency_result, ttl_result) = process_ssh_terminal(&mut buffer, address.clone(), title);

        latency.extend(latency_result);
        
        ttl.extend(ttl_result);
        
        
          
        //if iteration > 4 {
        (_min_value , _max_value) = find_min_max(&latency);
        
        _packet_loss = calculate_packet_loss(&latency);

        _latency_average = calculate_average_latency(&latency);

        _ttl_average = calculate_average_ttl(&ttl);

               
        // Move cursor to the bottom of the console (line 50)
        execute!(io::stdout(), cursor::MoveTo(1, BANNER_LINE))?;
        
        // Print a line at the bottom
        let banner_text = format!("\t\t{} \nAVG TTL: {} Max: {} ms Min: {} ms Actual: {:?} ms \n(Package Lost: {:.2}% of {:?}) AVG Latency: {:.3} ms", 
        title,
        _ttl_average,
        _max_value,
        _min_value,
        latency.last().unwrap_or(&0.0),  
        _packet_loss,
        latency.len(),
        _latency_average);
        

        print_line(&banner_text, BANNER_LINE)?;
        let buffer_str = str::from_utf8(&buffer).expect("Invalid UTF-8 data").to_string();
        
       if line_number < PING_RESULTS_END_LINE {
            print_line(&buffer_str, line_number)?;
            line_number += 1;
           
        } else {
            line_number = 0 ;
            clear_lines(PING_RESULTS_START_LINE, PING_RESULTS_END_LINE + 3 )?;
            
        }

        if latency.len() == 10 {
            clear_screen();
        }
        //Clean buffer
        buffer = [0; 4096];
        //Clean latency
        if latency.len() == 100 {
            // Pop the last 10 elements from the vector
            latency.remove(0);
        }

        })
}

/*
pub fn test()->  std::io::Error {
       
    let port: &str = "22";
    let title: String = "TD_Pto_Fibex".to_string();
    let destination_address: String = "8.8.8.8".to_string();
    let source_address: String = "45.182.140.24".to_string();
    let host: String = "10.0.0.8".to_string();
    let username: String = "nramirez".to_string();
    let password: String = "N3st0rR4m23*".to_string();
    //let on_live_test_or_report: String = "1".to_string();
    //let on_live_test_or_report: usize = on_live_test_or_report.parse().expect("Error");
    
    let command = format!("ping {} src-address={}", destination_address, source_address);
    
    // Convert host and port to a String
    let address = format!("{}:{}", host, port);
    let _address = format!("{}:{}", host, port);
    //let _check_ssh_config = check_ssh_config();
    
    
    let _session = establish_ssh_connection(&address, &username, &password)
        .map_err(|_err| {
            // Wrap the error in a custom std::io::Error
            io::Error::new(io::ErrorKind::Other, format!("No fue posible establecer sesion con el router: {} ",address))
        });


    //Time controled command 1
    let (sender, receiver) = mpsc::channel();
        
    let mut channel_result: Option<Result<Channel, ssh2::Error>> = None;

    // Spawn a new thread to execute the code
    let _handle = thread::spawn(move || {
        let _channel: Result<Channel, ssh2::Error> = _session.expect("No fue posible establecer el canal").channel_session();
        // Send the result through the channel when the task is done
        sender.send(_channel).expect("Error de conexion");
    });
    // Set the timeout duration
    let _timeout_duration = Duration::from_secs(30);
    // Get the current time
    let _start_time = Instant::now();

    
    // Wait for the thread to finish or until the timeout is reached
    match receiver.recv_timeout(_timeout_duration) {
        Ok(result) => {
            channel_result = Some(result);
                    }
        Err(_) => {
            // Timeout reached 
            clear_screen();
            println!("\t\tNo fue posible conectarse al router {} \n\t\t{}",address.clone(), title);
            let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
            process::exit(0);
        }
    }


    //Time controled command 2
    let (sender, receiver) = mpsc::channel();
            
     let _handle = thread::spawn(move || {
        let res = ping_test_continous_output(channel_result.expect("ss").unwrap(),  &title, address.clone(), &command).unwrap();

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
            println!("\t\tEl router {} se encuentra colgado",_address.clone());
            let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
            process::exit(0);
        }
    }
    let custom_error_message = "No fue posible conectarse al router";
    io::Error::new(io::ErrorKind::Other, custom_error_message)
  
   
}*/