use std::net::TcpStream;
use std::str;
use regex::Regex;
use ssh2::Session;
use std::io::{self, BufRead, Cursor};



fn ssh_continuous_output(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
    command: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the SSH server
    let tcp = TcpStream::connect((host, port))?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    // Authenticate with username and password
    session.userauth_password(username, password)?;
    if !session.authenticated() {
        return Err("Failed to authenticate".into());
    }

    // Open a channel and execute the command
    let mut channel = session.channel_session()?;
    channel.exec(command)?;

    // Receive and print the continuous output
    let mut buffer = [0; 4096];
    /*loop {
        let len = channel.read(&mut buffer)?;
        if len == 0 {
            // End of output
            break;
        }
        print!("{}", str::from_utf8(&buffer[..len])?);
    }*/
    process_ssh_terminal(&mut buffer);
    Ok(())
}

//fn process_ssh_terminal(stdout: &mut dyn BufRead) -> bool {
fn process_ssh_terminal(buffer: &mut [u8; 4096]) -> bool {
    let mut latency_avg: Vec<f64> = Vec::new();
    let mut ttl: Vec<i32> = Vec::new();
    let mut packet_loss_percentage = 0.0;
    let mut latency: Vec<f64> = Vec::new();
 
    // Add 0 value to average
    if latency_avg.is_empty() {
        latency_avg.push(0.0);
    }
 
    let mut combined_value = 0.0;
    let mut i = 0;
    let mut reader: BufReader<&mut [u8; 4096]> = BufReader::new(&stdout[..]);
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
        let clean_line = escape_ansi(&line);
        let line = clean_line;
 
        // Header line
        if line.contains("SEQ HOST") {
            continue;
        }
 
        // Averages line
        else if line.contains("sent=") && line.contains("received=") && line.contains("packet-loss=") {
            let elements: Vec<&str> = line.split_whitespace().collect();
 
            for element in elements {
                if element.contains("packet-loss=") {
                    packet_loss_percentage = element.split('=').nth(1).unwrap_or("0").trim_end_matches('%').parse().unwrap_or(0.0);
                } else if element.contains("avg-rtt=") {
                    combined_value = parse_latency_value(element);
                    latency_avg.push(combined_value);
                }
            }
        }
 
        // Ping value line
        else if line.trim().ends_with(&['m', 's', 'u', 's'][..]) {
            combined_value = parse_latency_value(&line);
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
 
        i += 1;
        if i == 3 {
            return true;
        }
    }
 
    false
 }
 
 fn parse_latency_value(element: &str) -> f64 {
    if element.contains("ms") && element.contains("us") {
        let value = element.split_whitespace().nth(4).unwrap_or_default();
        let (ms, us) = value.split_at(value.find("ms").unwrap_or_default());
        ms.parse::<f64>().unwrap_or_default() + us.trim_end_matches("us").parse::<f64>().unwrap_or_default() / 1000.0
    } else if element.contains("ms") {
        let value = element.split_whitespace().nth(4).unwrap_or_default();
        value.trim_end_matches("ms").parse::<f64>().unwrap_or_default()
    } else if element.contains("us") {
        let value = element.split_whitespace().nth(4).unwrap_or_default();
        value.trim_end_matches("us").parse::<f64>().unwrap_or_default() / 1000.0
    } else {
        0.0
    }
 }
 
 fn handle_error(line: &str) {
    println!("Could not socket");
    println!("#########           ERROR           #######");
    println!("{}", line);
 }

 

fn escape_ansi(line: &str) -> String {
    let ansi_escape = Regex::new(r"(\x9B|\x1B\[)[0-?]*[ -\/]*[@-~]").unwrap();
    let line_without_ansi = ansi_escape.replace_all(line, "");

    // Remove additional characters
    let line_without_extra_chars = line_without_ansi.replace(&['f', '\r', '\t', '\n'][..], "");

    line_without_extra_chars
}

fn main() {
    let host = "10.0.0.2";
    let port = 22; // Change it if your SSH server is running on a different port
    let username = "nramirez";
    let password = "N3st0rR4m23*";
    let command = "ping 8.8.8.8"; // Replace with the command you want to execute

    match ssh_continuous_output(host, port, username, password, command) {
        Ok(_) => println!("SSH connection successful"),
        Err(err) => eprintln!("Error: {}", err),
    }
}


