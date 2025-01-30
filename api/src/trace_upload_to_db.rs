use std::io::{self, BufRead, Cursor, BufReader};
use ssh2::Channel;
use std::error::Error as StdError;
use std::env;
use dotenv::dotenv;
use mysql_async::{Opts, OptsBuilder, Conn, Error};
use mysql_async::prelude::*;
use crate::ssh::establish_ssh_connection;
use std::io::Read;
use serde_json::{json, Value};
use std::collections::HashMap;

struct MySQLConnection {
    conn: Conn,
}

impl MySQLConnection {
    // Existing method to insert latency data
    pub async fn insert_latency_data(&mut self, test_id: i32, latency_ms: Option<f32>, packet_loss: Option<f32>, ttl: Option<i32>) -> Result<(), Error> {
        let query = "INSERT INTO latency_reports (test_id, latency_ms, packet_loss, ttl) VALUES (?, ?, ?, ?)";
        self.conn.exec_drop(query, (test_id, latency_ms, packet_loss, ttl)).await?;
        Ok(())
    }

    // New method to insert JSON data
    pub async fn insert_json_data(&mut self, test_id: i32, json_data: &Value) -> Result<(), Error> {
        let query = r#"
            INSERT INTO latency_reports_json (test_id, report_data)
            VALUES (?, ?)
        "#;
        self.conn.exec_drop(query, (test_id, json_data.to_string())).await?;
        Ok(())
    }
}

async fn upload_to_db(conn: &mut MySQLConnection, test_id: i32, json_data: &Value) -> Result<(), Error> {
    conn.insert_json_data(test_id, json_data).await?;
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

use tokio::time::{sleep, Duration};

pub async fn upload_mode(test_id: i32, username: &str, password: &str, address: &str, title: &str, command: &str) -> Result<(), Box<dyn StdError>> {
    loop {
        // Establish SSH connection
        let session = establish_ssh_connection(address.to_string(), username, password)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("Failed to establish SSH session: {}", err)))?;

        // Open a new channel
        let channel = session.channel_session()?;

        // Execute the command and process the output
        let _res = ping_test_continous_output(test_id, channel, title.to_string(), address.to_string(), command.to_string()).await?;

        // Wait for 30 seconds before rerunning the command
        sleep(Duration::from_secs(30)).await;
    }
}

fn process_ssh_terminal(buffer: &mut [u8; 4096]) -> Result<Value, Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(Cursor::new(&buffer[..]));
    let mut hubs = Vec::new();
    let mut current_hop: Option<HashMap<String, f32>> = None;

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

        // Skip header lines
        if line.starts_with("Columns:") || line.starts_with("#") {
            continue;
        }

        // Parse the structured data
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 8 {
            let count = parts[0].parse::<i32>().unwrap_or(0);
            let host = parts[1].to_string();
            let loss = parts[2].trim_end_matches('%').parse::<f32>().unwrap_or(0.0);
            let sent = parts[3].parse::<i32>().unwrap_or(0);
            let last = parts[4].trim_end_matches("ms").parse::<f32>().unwrap_or(0.0);
            let avg = parts[5].trim_end_matches("ms").parse::<f32>().unwrap_or(0.0);
            let best = parts[6].trim_end_matches("ms").parse::<f32>().unwrap_or(0.0);
            let worst = parts[7].trim_end_matches("ms").parse::<f32>().unwrap_or(0.0);
            let stdev = parts.get(8).and_then(|s| s.trim_end_matches("ms").parse::<f32>().ok()).unwrap_or(0.0);

            // Create a JSON object for the current hop
            let hub = json!({
                "count": count,
                "host": host,
                "Loss%": loss,
                "Snt": sent,
                "Last": last,
                "Avg": avg,
                "Best": best,
                "Wrst": worst,
                "StDev": stdev
            });

            hubs.push(hub);
        }
    }

    // Construct the final JSON structure
    let report = json!({
        "report": {
            "mtr": {
                "src": "traffic-report",
                "dst": "8.8.8.8", // Replace with the actual destination if available
                "tos": 0,
                "tests": 20, // Assuming 20 iterations
                "psize": "64",
                "bitpattern": "0x00"
            },
            "hubs": hubs
        }
    });

    Ok(report)
}
async fn ping_test_continous_output(test_id: i32, mut channel: Channel, _title: String, _address: String, command: String) -> Result<(), Box<dyn StdError>> {
    channel.exec(&command)?;

    let mut conn = connect_to_db().await?;
    let mut buffer = [0; 4096];

    loop {
        let len = channel.read(&mut buffer)?;
        if len == 0 {
            break;
        }

        // Process the SSH terminal output into JSON
        let report = process_ssh_terminal(&mut buffer)?;

        // Upload JSON data to the database
        upload_to_db(&mut conn, test_id, &report).await?;

        buffer = [0; 4096];
    }

    Ok(())
}