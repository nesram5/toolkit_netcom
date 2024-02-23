use std::fs::OpenOptions;
use std::error::Error;
use std::io::{self, BufRead, Cursor, BufReader, Read, Write};
use ssh2::Channel;
use crate::ssh::establish_ssh_connection;
use crate::auxiliar::{calculate_packet_loss, calculate_average_latency, parse_latency_value, return_path, clear_screen};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::process;

pub fn report_mode(username: &str, password: &str, address:String, title:String, command: String) ->  std::io::Error{
    let _address = address.clone();

    let _res = default_report(title.clone());

    let session = establish_ssh_connection(&address, username, password)
        .map_err(|err| {
            io::Error::new(io::ErrorKind::Other, format!("No fue posible conectarse al router: {}         {}",address, err))
        });
    let (sender, receiver) = mpsc::channel();
    // Spawn a new thread to execute the code
    let mut _channel_result: Option<Result<Channel, ssh2::Error>> = None;

    // Spawn a new thread to execute the code
    let _handle = thread::spawn(move || {
        let _channel = session.expect("No fue posible establecer el canal").channel_session();

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
            process::exit(0);
        }
    }

    //Time controled command 2
    let (sender, receiver) = mpsc::channel();
            
    let _handle = thread::spawn(move || {
       let res = process_and_save_report(_channel_result.expect("Error").unwrap(),  &title.clone(), &command).unwrap();

       sender.send(res).expect("Error de conexion");
   });

   let _timeout_duration = Duration::from_secs(130);
       // Get the current time
   let _start_time = Instant::now();
   
   match receiver.recv_timeout(_timeout_duration) {
       Ok(_res) => {
           println!("");
       }
       Err(_) => {
           // Timeout reached            clear_screen();
           println!("\t\tEl router {} se encuentra colgado",_address.clone());
           process::exit(0);
       }
   }
    
    let custom_error_message = "No fue posible conectarse al router";
    io::Error::new(io::ErrorKind::Other, custom_error_message)
}

fn process_and_save_report (mut channel: Channel, _title: &String, command: &String) -> Result<(), Box<dyn Error>>{
    let mut latency:Vec<f32> = Vec::new();
    let mut _latency_average:f32 = 0.0;
    let mut _packet_loss:f32 = 0.0;
    // Open a channel and execute the command0480
    let _channel: Result<(), ssh2::Error> = channel.exec(command);    
    let mut buffer = [0; 4096];
    let _result_report_process_ssh_terminal = Ok(loop {
            let _res = channel.read(&mut buffer);
            let latency_result = report_process_ssh_terminal(&mut buffer);

            latency.extend(latency_result);
            if latency.len() >= 100 {
                _packet_loss = calculate_packet_loss(&latency);
                _latency_average = calculate_average_latency(&latency);
                break;
            }
            //Clean buffer
            buffer = [0; 4096];
        });
    //Save report in a file
    let _result = save_report_to_txt_file(_title.to_string(), _latency_average ,_packet_loss);

    _result_report_process_ssh_terminal   
}


fn default_report(title: String) -> io::Result<()> {
    let path_report_file = return_path(&title, "cache"); 
    let mut file = OpenOptions::new().write(true).create(true).open(path_report_file)?;
    let _res = writeln!(file, "Error");
    let _res = writeln!(file, " Error");
    Ok(())
}

fn save_report_to_txt_file(title: String, latency_avg: f32, packet_lost_percentage:f32 ) -> io::Result<()> {
    let path_report_file = return_path(&title, "cache");
    let mut file = OpenOptions::new().write(true).create(true).open(path_report_file)?;
    if latency_avg > 45.0{
        writeln!(file, "{:.2} ms 🚨", latency_avg)?;
    }
    else {
        writeln!(file, "{:.2} ms", latency_avg)?;
    }
    if packet_lost_percentage != 0.0{
        writeln!(file, "{:.2} % PL 🚨", packet_lost_percentage)?;
    }
    else {
        writeln!(file, "")?;
    }
    
    Ok(())
}

fn report_process_ssh_terminal(buffer: &mut [u8; 4096]) -> Vec<f32>{
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
        if line.contains("SEQ HOST") {
            continue;
        } else if line.contains("avg-rtt=") && line.contains("packet-loss=") {
            continue;
        } else if line.trim().ends_with(&['m', 's', 'u', 's'][..]) {
            let combined_value = parse_latency_value(&line);
            latency.push(combined_value);
        } else if line.contains("could not...") || line.contains("packet-loss=100%") || line.contains("timeout") {
            latency.push(9999.0);
        }// else {
        //    latency.push(9999.0);
        //}

        if latency.len() != 0 {
            return latency;
        }
    }
    latency
}
/*
pub fn test_report(){
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
    report_mode(&username, &password,address, title.to_string(), command);
}
*/