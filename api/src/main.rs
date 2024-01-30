use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

use ssh2::Session;

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
    loop {
        let len = channel.read(&mut buffer)?;
        if len == 0 {
            // End of output
            break;
        }
        print!("{}", str::from_utf8(&buffer[..len])?);
    }

    Ok(())
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