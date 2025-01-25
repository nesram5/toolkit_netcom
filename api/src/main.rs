mod report_mode;
mod ping_test_mode;
mod ssh;
mod auxiliar;
mod upload_to_db;
use std::env;
use std::io;

use tokio; // Ensure you have tokio as a dependency in your Cargo.toml

//use ping_test_mode::test;

//use report_mode::test_report;

use crate::ssh::check_ssh_config;
use crate::report_mode::report_mode;
use crate::ping_test_mode::ping_test_mode;
use crate::upload_to_db::upload_mode;
#[tokio::main]
async fn start() ->  io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 8 {
        eprintln!("Usage: {} <title> <destination_address> <source_address> <host:port> <username> <password> <test_id> <on_live_test(1)_or_report(2)_or_upload(3)>", args[0]);
        std::process::exit(1);
    }
    //let port: &str = "22";
    let title: &String = &args[1];
    let destination_address: &String = &args[2];
    let source_address: &String = &args[3];
    //let host: &String = &args[4];
    let address: &str = &args[4];
    let username: &String = &args[5];
    let password: &String = &args[6];
    let test_id: &i32 = &args[7].parse().expect("Error");
    let on_live_test_or_report: &String = &args[8];
    let on_live_test_or_report: usize = on_live_test_or_report.parse().expect("Error");
    // Convert host and port to a String
    //let address = format!("{}:{}", host, port);

    let _check_ssh_config = check_ssh_config();

    match on_live_test_or_report {
        
    1 =>{
        let command: String = format!("ping {} src-address={}", destination_address, source_address);
       ping_test_mode(&username, &password,address, title, &command);
    }    
    2 => {
        let command: String = format!("ping {} src-address={}", destination_address, source_address);
        report_mode(&username, &password,address.to_string(), title.to_string().clone(), command.clone());
    }
    3 => {
        let command: String = format!("ping {} src-address={}", destination_address, source_address);
        let _res =upload_mode(*test_id, &username, &password,address, title, &command).await;
    }
    _ => {println!("Opcion Invalida")}
    }

    Ok(())
}

fn main(){
  match start() {
        Ok(()) => {
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("{}",e);
        }
   }
   //test_report();
}