use std::env;
use std::net::TcpStream;
use std::io::{self, BufRead, Cursor, BufReader, Read};
use ssh2::Session;
use std::process::Command;
use std::str;
use crossterm::{execute, cursor, terminal};

const BANNER_LINE: u16 = 20;
const PING_RESULTS_START_LINE: u16 = 1;
const PING_RESULTS_END_LINE: u16 = 15;

fn ssh_continuous_output(
    address: &String,
    username: &str,
    password: &str,
    command: &String,
    title: &String
) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the SSH server
    
    let tcp = TcpStream::connect(&address)?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    // Authenticate with username and password
    session.userauth_password(username, password)?;
    if !session.authenticated() {
        return Err("Failed to authenticate".into());
    }
    let mut latency:Vec<f32> = Vec::new();
    let mut ttl:Vec<i32> = Vec::new();
    let mut latency_average:f32 = 0.0;
    let mut packet_loss:f32 = 0.0;
    let mut min_value:f32 = 0.0;
    let mut max_value:f32 = 0.0;
    let mut ttl_average:i32 = 0;
        // Open a channel and execute the command0480
    let mut channel = session.channel_session()?;
    channel.exec(command)?;
    let mut buffer = [0; 512];
    let mut line_number = 1;
    let mut iteration = 0;
    clear_screen();

    Ok(loop {
        let len = channel.read(&mut buffer)?;
        if len == 0 {
            // End of output
            break;
        }
        
        // Process the continuous output
        //print!("{}", str::from_utf8(&buffer)?);
        let (latency_result, ttl_result) = process_ssh_terminal(&mut buffer);

        latency.extend(latency_result);
        
        ttl.extend(ttl_result);
        
          
        if iteration > 4 {
            (min_value , max_value) = find_min_max(&latency);
            
            packet_loss = calculate_packet_loss(&latency);

            latency_average = calculate_average_latency(&latency);

            ttl_average = calculate_average_ttl(&ttl);

            iteration = 0;
        }
        
        
        // Move cursor to the bottom of the console (line 50)
        execute!(io::stdout(), cursor::MoveTo(1, BANNER_LINE))?;
        
        // Print a line at the bottom
        let banner_text = format!("\t{} \nMax: {} ms Min: {} ms Actual: {:?} ms \nAVG TTL: {} Package Lost: {:.2}% AVG Latency: {:.3} ms", 
        title,
        max_value,
        min_value,
        latency.last().unwrap_or(&0.0),                    
        ttl_average,
        packet_loss,
        latency_average);

        print_line(&banner_text, BANNER_LINE)?;
        let buffer_str = str::from_utf8(&buffer).expect("Invalid UTF-8 data");
        
       if line_number < PING_RESULTS_END_LINE {
            
            print_line(&buffer_str, line_number)?;
            line_number += 1;
           
        } else {
            line_number = 0 ;
            clear_lines(PING_RESULTS_START_LINE, PING_RESULTS_END_LINE + 3 )?;
            
        }

        iteration += 1;
        //Clean buffer
        buffer = [0; 512];
        //Clean latency
        if latency.len() == 100 {
            // Pop the last 10 elements from the vector
            latency.remove(0);
        }

        })

    }


fn process_ssh_terminal(buffer: &mut [u8; 512]) -> (Vec<f32>, Vec<i32>){
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
            latency.push(0.0);
            println!("{}", line);

        }
        // Unknown line
        else {
            //println!("Unknown Line:");
            latency.push(0.0);
            println!("{}", line);
        }
 
        
        if latency.len() != 0 {
            return (latency , ttl);
        }
    }
    (latency, ttl)

}
 
 fn parse_latency_value(element: &str) -> f32 {
    if element.contains("ms") && element.contains("us") {
        let value = element.split_whitespace().nth(4).unwrap_or_default();
        let (ms, us) = value.split_at(value.find("ms").unwrap_or_default());
        ms.parse::<f32>().unwrap_or_default() + us.trim_end_matches("us").parse::<f32>().unwrap_or_default() / 1000.0
    } else if element.contains("ms") {
        let value = element.split_whitespace().nth(4).unwrap_or_default();
        value.trim_end_matches("ms").parse::<f32>().unwrap_or_default()
    } else if element.contains("us") {
        let value = element.split_whitespace().nth(4).unwrap_or_default();
        value.trim_end_matches("us").parse::<f32>().unwrap_or_default() / 1000.0
    } else {
        0.0
    }
 }
 
fn print_line(content: &str, line: u16) -> io::Result<()> {
    execute!(
        io::stdout(),
        cursor::MoveTo(1, line),
        terminal::Clear(terminal::ClearType::CurrentLine)
    )?;
    println!("{}", content);
    Ok(())
}

fn clear_lines(start: u16, end: u16) -> io::Result<()> {
    for line in start..=end {
        execute!(io::stdout(), cursor::MoveTo(1, line), terminal::Clear(terminal::ClearType::CurrentLine))?;
    }
    Ok(())
}

fn clear_screen() {
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

fn calculate_packet_loss(latencies: &Vec<f32>) -> f32 {
    // Count the number of latencies equal to or less than the threshold
    let lost_packets = latencies.iter().filter(|&&latency| latency <= 0.0).count() as f32;

    // Calculate the packet-loss percentage
    let total_packets = latencies.len() as f32;
    let packet_loss_percentage = (lost_packets * 100.00) / total_packets;

    packet_loss_percentage
}

fn calculate_average_latency(latencies: &Vec<f32>) -> f32 {
    // Check if the vector is not empty
    if latencies.is_empty() {
        return 0.0;
    }

    // Sum up all latency values
    let sum: f32 = latencies.iter().sum();

    // Calculate the average latency
    let average_latency = sum / latencies.len() as f32;

    average_latency
}

fn calculate_average_ttl(values: &Vec<i32>) -> i32 {
    // Check if the vector is not empty
    if values.is_empty() {
        return 0;
    }

    // Sum up all values
    let sum: i32 = values.iter().sum();

    // Calculate the average and truncate the decimal part
    let average = sum / values.len() as i32;

    average
}
fn find_min_max(latencies: &Vec<f32>) -> (f32 , f32) {
       // Find the minimum and maximum values
    let min_value = *latencies.iter().min_by(|&a, &b| a.partial_cmp(b).unwrap()).unwrap();
    let max_value = *latencies.iter().max_by(|&a, &b| a.partial_cmp(b).unwrap()).unwrap();

    return (min_value, max_value);
}

fn test(){
    // Extract values from arguments
    let port: &str = "22";
    let title = "TD_Int_Dayco-Parques".to_string();
    let destination_address = "172.16.0.121";
    let source_address = "172.16.0.122";
    let host = "10.10.48.1";
    let username = "nramirez";
    let password = "N3st0rR4m23*";    
    let command = format!("ping {} src-address={}", destination_address, source_address) ; // Replace with the command you want to execute
    // Convert host and port to a String
    let address = format!("{}:{}", host, port);
    match ssh_continuous_output(&address, username, password, &command, &title) {
        Ok(_) => println!("SSH connection successful"),
        Err(err) => eprintln!("Error: {}", err),
    }

}

fn main_1(){
    let args: Vec<String> = env::args().collect();

    if args.len() < 7 {
        eprintln!("Usage: {} <title> <host> <username> <password> <source_address> <destination_address>", args[0]);
        std::process::exit(1);
    }
    let port: &str = "22";
    let title = &args[1];
    let destination_address = &args[2];
    let source_address = &args[3];
    let host = &args[4];
    let username = &args[5];
    let password = &args[6];
    
    let command = format!("ping {} src-address={}", destination_address, source_address);
    
    // Convert host and port to a String
    let address = format!("{}:{}", host, port);


    match ssh_continuous_output(&address, username, password, &command, &title) {
        Ok(_) => println!("SSH connection successful"),
        Err(err) => eprintln!("Error: {}", err),
    }
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

fn main() {
    
    main_1();

}