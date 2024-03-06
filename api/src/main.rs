mod report_mode;
mod ping_test_mode;
mod ssh;
mod auxiliar;
use std::env;
use std::io;
//use ping_test_mode::test;

//use report_mode::test_report;

use crate::ssh::check_ssh_config;
use crate::report_mode::report_mode;
use crate::ping_test_mode::ping_test_mode;

fn start() ->  io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 8 {
        eprintln!("Usage: {} <title> <host_port> <username> <password> <source_address> <destination_address> <on_live_test_or_report>", args[0]);
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
    let on_live_test_or_report: &String = &args[7];
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