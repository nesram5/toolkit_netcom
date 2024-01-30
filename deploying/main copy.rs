use regex::Regex;
//use std::io::{BufRead};
use std::io::Read;
use std::net::TcpStream;
use ssh2::{Session, Channel};
use ssh2::Error as SshError;
use std::io::Error as IoError;
use std::convert::From;

//pub fn read_message<R>(_read: &mut R)
//where
//    R: ?Sized + BufRead,
//{}

fn escape_ansi(line: &str) -> String {
    let ansi_escape = Regex::new(r"(\x9B|\x1B\[)[0-?]*[ -\/]*[@-~]").unwrap();
    let line_without_ansi = ansi_escape.replace_all(line, "");

    // Remove additional characters
    let line_without_extra_chars = line_without_ansi.replace(r"\f", "")
        .replace("\r", "")
        .replace("\t", "")
        .replace("\n", "");

    line_without_extra_chars
}
// &mut dyn BufRead
/*
fn process_ssh_terminal(stdout: Vec <u8>) -> bool {
    let mut latency_avg: Vec<f64> = Vec::new();
    let mut ttl: Vec<i32> = Vec::new();
    let mut packet_loss_percentage: f64 = 0.0;
    let mut latency: Vec<f64> = Vec::new();
    
    //Add 0 value to average
    if latency_avg.len() == 0 {
        latency_avg.push(0.0);
    }
    
    let mut combined_value: f64 = 0.0;
    let mut i: i32 = 0;
    
    loop {
        let mut line = String::new();
        if let Ok(bytes_read) = stdout.read_to_end (&mut line) {
            if bytes_read == 0 {
                break;
            }
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
                    packet_loss_percentage = element.split('=').nth(1).unwrap().trim_end_matches('%').parse().unwrap();
                }
                else if element.contains("avg-rtt=") {
                    if element.contains("ms") && element.contains("us") {
                        let value = element.split_whitespace().nth(4).unwrap();
                        let (ms, us) = value.split_at(value.find("ms").unwrap());
                        combined_value = ms.parse::<f64>().unwrap() + us.trim_end_matches("us").parse::<f64>().unwrap() / 1000.0;
                    }
                    else if element.contains("ms") {
                        let value = element.split_whitespace().nth(4).unwrap();
                        combined_value = value.trim_end_matches("ms").parse().unwrap();
                    }
                    else if element.contains("us") {
                        let value = element.split_whitespace().nth(4).unwrap();
                        combined_value = value.trim_end_matches("us").parse::<f64>().unwrap() / 1000.0;
                    }
                    latency_avg.push(combined_value);
                }
            }
        }
        
        // Ping value line
        else if line.trim().ends_with(&['m', 's', 'u', 's'][..]) {
            if line.contains("ms") && line.contains("us") {
                let values: Vec<&str> = line.split_whitespace().collect();
                if values.len() >= 5 {
                    let value = values[4];
                    let (ms, us) = value.split_at(value.find("ms").unwrap());
                    combined_value = ms.parse::<f64>().unwrap() + us.trim_end_matches("us").parse::<f64>().unwrap() / 1000.0;
                    latency.push(combined_value);
                }
            }
            else if line.contains("ms") {
                let values: Vec<&str> = line.split_whitespace().collect();
                if values.len() >= 5 {
                    let value = values[4];
                    combined_value = value.trim_end_matches("ms").parse().unwrap();
                    latency.push(combined_value);
                }
            }
            else if line.contains("us") {
                let values: Vec<&str> = line.split_whitespace().collect();
                if values.len() >= 5 {
                    let value = values[4];
                    combined_value = value.trim_end_matches("us").parse::<f64>().unwrap() / 1000.0;
                    latency.push(combined_value);
                }
            }
            
            //Grab TTL value
            if line.len() >= 5 {
                ttl.push(line.split_whitespace().nth(3).unwrap().parse().unwrap());
            }
        }
        
        else if line.contains("could not...") {
            println!("Could not socket\n");
            println!("#########           ERROR           #######\n");
        }
        
        else if line.contains("packet-loss=100%") {
            let elements: Vec<&str> = line.split_whitespace().collect();
            
            for element in elements {
                if element.contains("packet-loss=") {
                    packet_loss_percentage = element.split('=').nth(1).unwrap().trim_end_matches('%').parse().unwrap();
                }
            }
            println!("Could not socket\n");
            println!("#########           ERROR           #######\n");
        }
        
        else if line.contains("timeout") {
            println!("Could not socket\n");
            println!("#########           ERROR           #######\n");
        }
        
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
} */

fn connection_mikrotik(_title: &str, _destination_ip: &str, _source_ip: &str, hostname: &str, username: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Replace these values with your SSH server details
    
    // Connect to the SSH server
    let _tcp = std::net::TcpStream::connect(format!("{}:22", hostname))?;
    let mut sess = Session::new()?;
    // sess.handshake(&tcp)?;
    sess.handshake()?;

    // Authenticate with the server using password
    sess.userauth_password(username, password)?;

    // Start the channel and execute the ping command
    let mut channel = sess.channel_session()?;
    channel.exec("ping 8.8.8.8 src-address=45.182.141.88")?;

    // Read the output of the command
    let mut output = Vec::new();
    channel.read_to_end(&mut output)?;

    // Print the output
    let result = String::from_utf8_lossy(&output);
    
    //process_ssh_terminal(output);

    println!("{}", result);
    print!("Hello wold");
    Ok(())
}

fn connection_octus(_title: &str, _destination_ip: &str, _source_ip: &str, hostname: &str, username: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Replace these values with your SSH server details
    
    // Connect to the SSH server
    let _tcp = std::net::TcpStream::connect(format!("{}:22", hostname))?;
    let mut sess = Session::new()?;
    sess.handshake()?;
    // sess.handshake(&tcp)?;

    // Authenticate with the server using password
    sess.userauth_password(username, password)?;

    // Start the channel and execute the ping command
    let mut channel = sess.channel_session()?;
    channel.exec("ping 8.8.8.8")?;

    /*
    Read the output of the command
    */
    let mut output = Vec::new();
    channel.read_to_end(&mut output)?;

    // Print the output
    let result = String::from_utf8_lossy(&output);
    println!("{}", result);
    println!("hello");
    Ok(())
}


fn wich_model(title: &str, destination_ip: &str, source_ip: &str, hostname: &str, username: &str, password: &str) {

    if hostname == "10.1.2.2" {
        let _ = connection_octus(title, destination_ip, source_ip, hostname, username, password);
    } else {
        let _ = connection_mikrotik(title, destination_ip, source_ip, hostname, username, password);
    }
}
 // let args: Vec<String> = env::args().collect();
// ///     if args.len() >= 7 {
// ///         wich_model(&args[1], &args[2], &args[3], &args[4], &args[5], &args[6]);
// ///     


fn connect_ssh(server: &str, username: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create a new SSH session
    let tcp = std::net::TcpStream::connect(format!("{}:22", server))?;
    let mut sess = Session::new()?;
    
    // Handshake and authenticate with the server
    sess.handshake()?;
    sess.userauth_password(username, password)?;
    
    // Check if the authentication was successful
    if !sess.authenticated() {
        return Err(Box::from("Failed to authenticate"));
    }
    
    // Open a channel for executing commands on the server
    let mut channel = sess.channel_session()?;
    
    // Request a shell and execute a command
    channel.request_pty("xterm", None, None)?;
    channel.shell()?;
    channel.write("ls\n".as_bytes())?;
    
    // Read the output of the command
    let mut buffer = vec![0; 1024];
    let nbytes = channel.read(&mut buffer)?;
    println!("{}", std::str::from_utf8(&buffer[..nbytes])?);
    
    // Close the channel and disconnect from the server
    channel.close()?;
    sess.disconnect(None, "Bye bye", None)?;
    
    Ok(())
}


fn execute_ssh_command(username: &str, password: &str, hostname: &str, command: &str) -> Result<String, ssh2::Error> {
    let tcp = TcpStream::connect(format!("{}:22", hostname))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_password(username, password)?;

    let mut channel = sess.channel_session()?;
    channel.exec(command)?;
    
    let mut output = String::new();
    channel.read_to_string(&mut output)?;
    
    channel.wait_close()?;
    let exit_status = channel.exit_status()?;
    
    Ok(output)
}
fn main() {
// let result = connection_mikrotik("hello", "8.8.8.8" , "45.182.141.88", "10.0.0.8", "nramirez", "N3st0rR4m23*");
match execute_ssh_command("10.0.0.8", "nramirez", "N3st0rR4m23*", "ping 8.8.8.8") {
    Ok(_) => println!("Connected to SSH server successfully"),
    Err(e) => println!("Failed to connect: {}", e),
}
}


