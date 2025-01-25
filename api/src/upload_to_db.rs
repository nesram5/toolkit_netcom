use std::io::{self, BufRead, Cursor, BufReader};
use ssh2::Channel;
use std::process::Command;
use std::error::Error as StdError;
use std::process;
use std::env;
use dotenv::dotenv;
use mysql_async::{Opts, OptsBuilder, Conn, Error};
use mysql_async::prelude::*;
use crate::ssh::establish_ssh_connection;
use crate::auxiliar::parse_latency_value;
use std::io::Read;

struct MySQLConnection {
    conn: Conn,
}

impl MySQLConnection {
    pub async fn new(opts: Opts) -> Result<Self, Error> {
        let conn = Conn::new(opts).await?;
        Ok(MySQLConnection { conn })
    }

    pub async fn insert_latency_data(&mut self, test_id: i32, latency_ms: f32) -> Result<(), Error> {
        let query = "INSERT INTO latency_reports (test_id, latency_ms) VALUES (?, ?)";
        self.conn.exec_drop(query, (test_id, latency_ms)).await?;
        Ok(())
    }
}

async fn upload_to_db(conn: &mut MySQLConnection, test_id: i32, latency_ms: &f32) -> Result<(), Error> {
    conn.insert_latency_data(test_id, *latency_ms).await?;
    Ok(())
}

async fn connect_to_db() -> Result<MySQLConnection, Error> {
    dotenv().ok(); // Load environment variables from .env file

    // Read variables from the environment
    let hostname = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let username = env::var("DB_USERNAME").unwrap_or_else(|_| "root".to_string());
    let password = env::var("DB_PASSWORD").unwrap_or_else(|_| "password".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "test_db".to_string());

    let opts = OptsBuilder::default()
        .ip_or_hostname(hostname)
        .user(Some(username))
        .pass(Some(password))
        .db_name(Some(db_name))
        .into();

    let conn = MySQLConnection::new(opts).await?;
    println!("Connected to database");
    Ok(conn)
}

pub async fn upload_mode(test_id: i32, username: &str, password: &str, address: &str, title: &str, command: &str) -> Result<(), Box<dyn StdError>> {
    let mut i = 0;
    loop {
        let session = establish_ssh_connection(address.to_string(), username, password)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("Failed to establish SSH session: {}", err)))?;

        let channel = session.channel_session()?;

        let _res = ping_test_continous_output(test_id, channel, title.to_string(), address.to_string(), command.to_string()).await?;

        i += 1;
        if i == 3 {
            break;
        }
    }

    Ok(())
}

fn process_ssh_terminal(buffer: &mut [u8; 4096], address: String, title: String) -> (Vec<f32>, Vec<i32>) {
    let mut ttl: Vec<i32> = Vec::new();
    let mut latency: Vec<f32> = Vec::new();
    let mut reader = BufReader::new(Cursor::new(&buffer[..]));

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

            if line.len() >= 5 {
                ttl.push(line.split_whitespace().nth(3).unwrap_or("0").parse().unwrap_or(0));
            }
        } else if line.contains("could not...") || line.contains("packet-loss=100%") || line.contains("timeout") {
            latency.push(9999.0);
            println!("{}", line);
        } else if line.contains("Invalid ar...") {
            println!("\t\tInvalid command, check if the interface or IP is in use on router {} \n\t\tTest name: {}", address, title);
            let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
            process::exit(0);
        }

        if !latency.is_empty() {
            return (latency, ttl);
        }
    }
    (latency, ttl)
}

async fn ping_test_continous_output(test_id: i32, mut channel: Channel, title: String, address: String, command: String) -> Result<(), Box<dyn StdError>> {
    let mut latency: Vec<f32> = Vec::new();
    channel.exec(&command)?;

    let mut conn = connect_to_db().await?;
    let mut buffer = [0; 4096];

    loop {
        let len = channel.read(&mut buffer)?;
        if len == 0 {
            break;
        }

        let (latency_result, _ttl_result) = process_ssh_terminal(&mut buffer, address.to_string(), title.to_string());
        latency.extend(latency_result);

        if let Some(last_latency) = latency.last() {
            if *last_latency != 0.0 {
                upload_to_db(&mut conn, test_id, last_latency).await?;
            }
        }

        buffer = [0; 4096];

        if latency.len() == 100 {
            latency.remove(0);
        }
    }

    Ok(())
}