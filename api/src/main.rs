use std::net::TcpStream;
use std::env;
use std::io::{self, BufRead, Cursor, BufReader, Read};
use ssh2::Session;
use std::process::Command;
use std::str;
//use std::{thread, time};
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

    let mut latency: Vec<Vec<f32>> = Vec::new();
    let mut ttl: Vec<Vec<i32>> = Vec::new();
    let mut latency_avg: Vec<String> = Vec::new();
    let mut packet_loss: Vec<String> = Vec::new();

    // Open a channel and execute the command0480
    let mut channel = session.channel_session()?;
    channel.exec(command)?;
    let mut buffer = [0; 4096];
    let mut line_number = 1;
    
    clear_screen();

    Ok(loop {
        let len = channel.read(&mut buffer)?;
        if len == 0 {
            // End of output
            break;
        }
        
        // Process the continuous output
        //print!("{}", str::from_utf8(&buffer)?);
        let (latency_avg_str, latency_result, ttl_result, packet_lost_percentage_string) = process_ssh_terminal(&mut buffer);

              /*println!(
            "AVG {:?}, Latency {:?}, TTL {:?}, PL {:?}",
            latency_avg, latency, ttl, packet_lost_percentage
        );
        //clear_console();
        // Clear the buffer
        */
        latency.push(latency_result);//f32
        ttl.push(ttl_result);//i32
        latency_avg.push(latency_avg_str);//String
        packet_loss.push(packet_lost_percentage_string);//String
       

        let latency_int: Vec<i64> = custom_round(latency.clone());

        let max_value = *latency_int.iter().max().unwrap_or(&0);

        let min_value = *latency_int.iter().min().unwrap_or(&0);
        

        // Move cursor to the bottom of the console (line 50)
        execute!(io::stdout(), cursor::MoveTo(1, BANNER_LINE))?;
        
        // Print a line at the bottom
        let banner_text = format!("{} \nMax: {} ms Min: {} ms Actual: {:?} ms \nAVG TTL: {:?} Package Lost: {:?} ms AVG: {:?} ms", 
        title,
        max_value,
        min_value,
        latency.last().unwrap(),                    
        ttl.last().unwrap(),
        packet_loss.last().unwrap(),
        latency_avg.last().unwrap());

        print_line(&banner_text, BANNER_LINE)?;
        let buffer_str = str::from_utf8(&buffer).expect("Invalid UTF-8 data");
        

       if line_number < PING_RESULTS_END_LINE {
            
            print_line(&buffer_str, line_number)?;
            line_number += 1;
            //thread::sleep(time::Duration::from_millis(2));
           
            
           
        }
       
        else {
            line_number = 0 ;
            //thread::sleep(time::Duration::from_millis(5));
            clear_lines(PING_RESULTS_START_LINE, PING_RESULTS_END_LINE + 3 )?;
            
        }
        // Sleep for a while to keep the result visible
        //thread::sleep(time::Duration::from_secs(1));
        //clean buffer
        if buffer.iter().filter(|&&c| c != 0).count() >= 4096 {
            // Buffer is full, do something
            // Clear the buffer
            buffer = [0; 4096];
        }
        })
    }


fn process_ssh_terminal(buffer: &mut [u8; 4096]) -> (String, Vec<f32>, Vec<i32>, String){
    let mut ttl: Vec<i32> = Vec::new();
    let mut latency: Vec<f32> = Vec::new();
    let avg_rtt = String::new();
    let packet_loss = String::new();

    // Add 0 value to average
    /*if latency_avg.is_empty() {
        latency_avg.push(0.0);
    }
    */
    
    //let mut i = 0;
    let mut reader = BufReader::new(Cursor::new(&mut buffer[..]));

    // Add implementation for escape_ansi function or remove its usage

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

            let tokens: Vec<&str> = line.split_whitespace().collect();

            // Find the values of packet-loss and avg-rtt
            let packet_loss_str: Option<&str> = tokens.iter().find(|&&token| token.starts_with("packet-loss=")).map(|token| &token[12..]);
            let avg_rtt_str: Option<&str> = tokens.iter().find(|&&token| token.starts_with("avg-rtt=")).map(|token| &token[8..]);
        
            // Convert Option<&str> to String
            let packet_loss = packet_loss_str.map_or_else(|| String::from(""), |s| s.to_string());
            let avg_rtt = avg_rtt_str.map_or_else(|| String::from(""), |s| s.to_string());
        
            return (avg_rtt, latency , ttl, packet_loss);
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
            handle_error(&line);
        }
 
        // Unknown line
        else {
            println!("Unknown Line:");
            println!("{}", line);
        }
 
        
        if latency.len() != 0 {
            return (avg_rtt, latency , ttl, packet_loss);
        }
    }
    (avg_rtt, latency, ttl, packet_loss)
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
 
 fn handle_error(line: &str) {
    println!("Could not socket");
    println!("#########           ERROR           #######");
    println!("{}", line);
 }

 fn custom_round(latency: Vec<Vec<f32>>) -> Vec<i64> {
    latency.into_iter().flat_map(|inner_vec| {
        inner_vec.into_iter().map(|x| {
            if x >= 0.0 {
                (x + 0.5) as i64
            } else {
                (x - 0.5) as i64
            }
        })
    }).collect()
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
fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if enough arguments are provided
    if args.len() < 7 {
        eprintln!("Usage: {} <title> <host> <username> <password> <source_address> <destination_address>", args[0]);
        std::process::exit(1);
    }

    // Extract values from arguments
    let port: &str = "22";
    let title = &args[1];
    let destination_address = &args[2];
    let source_address = &args[3];
    let host = &args[4];
    let username = &args[5];
    let password = &args[6];
    
    
    let command = format!("ping {} src-address={}", destination_address, source_address) ; // Replace with the command you want to execute
    
    // Convert host and port to a String
    let address = format!("{}:{}", host, port);


    match ssh_continuous_output(&address, username, password, &command, &title) {
        Ok(_) => println!("SSH connection successful"),
        Err(err) => eprintln!("Error: {}", err),
    }
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}