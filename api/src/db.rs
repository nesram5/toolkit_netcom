use mysql_async::{Conn, Error, Opts, OptsBuilder};
use mysql_async::prelude::*;
use tokio;

struct MySQLConnection {
    conn: Conn,
}

impl MySQLConnection {
    // Connect to the MySQL database
    pub async fn new(opts: Opts) -> Result<Self, Error> {
        let conn = Conn::new(opts).await?;
        Ok(MySQLConnection { conn })
    }

    // Insert latency data into the latency_reports table
    pub async fn insert_latency_data(&mut self, test_id: i32, latency_ms: f64) -> Result<(), Error> {
        let query = "INSERT INTO latency_reports (test_id, latency_ms) VALUES (?, ?)";
        self.conn.exec_drop(query, (test_id, latency_ms)).await?;
        Ok(())
    }
}


pub async fn upload_to_db(conn: &mut MySQLConnection, test_id: i32, latency_ms: f64) -> Result<(), Error> {
  

    // Insert data into the latency_reports table
    conn.insert_latency_data(test_id, latency_ms).await?;
    println!("Latency data inserted successfully!");

    Ok(())
}

pub async fn connect_to_db() -> Result<(), MySQLConnection> {
    // Database connection configuration
    let opts = OptsBuilder::new()
        .ip_or_hostname("localhost")
        .user(Some("nramirez"))
        .pass(Some("N3tc0m++"))
        .db_name(Some("latencies"))
        .into();

    // Create a new connection
    let mut conn = MySQLConnection::new(opts).await?;
    Ok((conn))
}