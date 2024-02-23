use ssh2::Session;
use std::net::TcpStream;
use std::fs;
use std::io::{self, Write};


pub fn establish_ssh_connection(address: &String,username: &str, password: &str) -> Result<Session, Box<dyn std::error::Error>> {
    let _tcp = TcpStream::connect(address)?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(_tcp);
    sess.handshake()?;
    // Password-based authentication
    //sess.host_key();
    //let _know_host = sess.known_hosts();
    sess.userauth_password(username, password)?;
    if !sess.authenticated() {
        return Err("Credenciales invalidas".into());
    }
    Ok(sess)
}

pub fn check_ssh_config() -> io::Result<()> {
    // Get the path to the user's SSH config file
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Unable to determine home directory");
            return Ok(());
        }
    };
    let ssh_config_file = home_dir.join(".ssh").join("config");
    let ssh_config_path = home_dir.join(".ssh");
    if !ssh_config_file.exists() {
    // Check if the config file exists
        if !ssh_config_path.exists() {
            // Create the .ssh directory if it doesn't exist
            fs::create_dir_all(ssh_config_path.parent().unwrap())?;
        }   
        // Create and write the content to the config file
        let mut file = fs::File::create(&ssh_config_file)?;
        write!(file, "Host *\n\tStrictHostKeyChecking no\n\tUserKnownHostsFile=/dev/null")?;

        
    }
     else {
        println!("");
    }

    Ok(())
}
