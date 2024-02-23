//find_ip.rs

use std::io::{self, BufRead, BufReader, Write};
use std::process::Command;
use std::collections::HashSet;
use std::fs::File;
use crate::auxiliar::{check_default_ro_to, clear_screen, print_first_five_lines, read_json_file_to_vec, return_path, sort_ip_addresses};
use crate::ssh::{establish_ssh_connection,send_ssh_command_save_in_txt};
use crate::data;


pub fn option_3(username:&str, password: &str ){
    let mut _router_segment:Vec<String> = Vec::new();
    let port: &str = "22";
    let (mut _segment1, mut _segment2, mut _host) = ip_segment_node_choice();
                _router_segment.push(_segment1.to_string());
                _router_segment.push(_segment2.to_string());

    let _address = format!("{}:{}", _host, port).to_string();
    //FIND IP In Use first
    let mut looking_for: &str = "gateway=";
    let mut iterator = 0;
    let mut _ip_in_use:Vec<String> = Vec::new();
    while iterator <= 4 {
            let ssh_command = format!("ip route print terse without-paging where gateway~\"10.{}.{}\"\r\n",_router_segment[0], _router_segment[1]).to_string();
            iterator += 1;
            let _segment2_i32: i32 = _segment2.parse().unwrap(); // Convert the string to an integer
            let result = _segment2_i32 + iterator;
            _router_segment.insert(1, result.to_string());

            _ip_in_use = find_ip_or_segment_in_use(&ssh_command, &_address, &username, &password, looking_for).unwrap_or_default();            
    }
    //Find available_ip
    let _find_available_management_ip = find_available_management_ip(_ip_in_use, _segment1, _segment2);

    //FIND Segments in use
    looking_for = "dst-address=";
    //_host = "10.0.0.8".to_string(); //Reflector
    //let _address = format!("{}:{}", _host, port).to_string();
    let _address = check_default_ro_to("segment".to_string()).unwrap();
    let ssh_command: String = "ip route print terse without-paging where dst-address~\"192.168\"\r\n".to_string();
    let _segment_in_use = find_ip_or_segment_in_use(&ssh_command, &_address, &username, &password, looking_for).unwrap_or_default();
    clear_screen();          
    //Find available_Segments
    let _available_segments = find_available_segments(_segment_in_use);
    println!("Aqui tienes 5 IP de gestion disponibles\n");
    //Print Results
    let path_available_ip = return_path("available_IP.txt", "cache");
    match print_first_five_lines(path_available_ip.to_str().unwrap_or_default()){
        Ok(_) => {
            println!("\nRecuerda verificarlos en las plataformas como WispHub, PRTG y pruebas de ping\n");
            println!("Para ver la lista completa consulta el archivo /cache/available_IP.txt\n")
        },
        Err(err) => eprintln!("Error: {}", err),
    }

    println!("\nAqui tienes 5 segmentos disponibles\n");

    fn print_first_five_segments(file_path: &str) -> io::Result<()> {
        // Open the file
        let file = File::open(file_path)?;
    
        // Create a buffered reader to read the file line by line
        let reader = BufReader::new(file);
    
        // Use an iterator to iterate over the lines
        for (line_number, line) in reader.lines().enumerate() {
            // Check if we've printed 5 lines already
            if line_number >= 1496 && line_number <= 1501{
                println!("{}", line?);
                 // Exit the loop once we've printed 5 lines
            } else if line_number == 1502 {
                break;
            }
        }
    
        Ok(())
    }

    let path_available_segments = return_path("available_segments.txt", "cache");
    match print_first_five_segments(path_available_segments.to_str().unwrap_or_default()){
        Ok(_) => println!("\nPara ver la lista completa consulta el archivo /cache/available_segments.txt\n"),
        Err(err) => eprintln!("Error: {}", err),
    }
    //Pause for read
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

fn ip_segment_node_choice() -> (String, String, String){
    
    clear_screen();

    let path_rf_nodes = return_path("rf_nodes.json", "data");
    let mut ip_segment_node:Vec<(String, Vec<String>)> = Vec::new()
;    match read_json_file_to_vec(path_rf_nodes) {
        Ok(vec_data) => {
        ip_segment_node = vec_data;
        },
        Err(err) => eprintln!("Error: {}", err),
    }
    let mut _number: String = String::new();
    let mut _segment: String = String::new();
    let mut _ip: String = String::new();
    loop {
    
    
        println!("Selecione el router en el que desea buscar IP disponible:\n");
        for (i, (name, _)) in ip_segment_node.iter().enumerate() {
            println!("{}. {}", i + 1, name);
        }

        println!("\nIngrese un numero: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let selected_router_index: usize = match input.trim().parse::<usize>() {
            Ok(num) => num - 1,
            Err(_) => {
                println!("Seleccion Invalida");
                break;
            }
        };
        
        if let Some((_, values)) = ip_segment_node.get(selected_router_index) {
            if let [number, segment, ip] = values.as_slice(){
                _number = number.to_string();
                _segment = segment.to_string();
                _ip = ip.to_string();
                
                println!("\nBuscando IP y Segmento /29 en el nodo seleccionado");
                break;
               }   
            
        } else {
            println!("Seleccion invalida");
            let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
            clear_screen();
        }
    }
   (_number, _segment, _ip)
}


fn find_available_segments(_segment_in_use:Vec<String>) -> io::Result<()> {
    

    // Generate the complete set of /29 IPs
    let complete_ip: HashSet<String> = data::complete_ip_segments();
    
    //Convert Vec String to Hashset
    let in_use_ip_segments: HashSet<String> = _segment_in_use.into_iter().collect();

    // Identify the missing /29 IPs
    let available_ip_segments: HashSet<String> = complete_ip
        .difference(&in_use_ip_segments)
        .map(|s| s.clone())
        .collect();

    let mut sorted_vec: Vec<String> = available_ip_segments.clone().into_iter().collect();
    sorted_vec = sort_ip_addresses(sorted_vec); // Sort the Vec
    let filename = return_path("available_segments.txt", "cache");
    let mut file = File::create(filename)?;
    for element in sorted_vec {
        writeln!(file, "{}", element)?;
    }

    Ok(())
}

fn find_available_management_ip( find_ip_or_segment_in_use:Vec<String>, middle_octet: String, node_segment_initial: String) -> io::Result<()>  {
    let node_segment_initial_u8: u8 = match node_segment_initial.parse() {
        Ok(parsed) => parsed,
        Err(_) => {            
            println!("Failed to parse the string as u8. Using default value.");
            0 
        }
    };
    let node_segment_final = node_segment_initial_u8 + 4;

    // Generate the complete set of IPs
    let j_range = node_segment_initial_u8..node_segment_final;
    let k_range = 130..255;

    // Create a HashSet to store the IP _addresses
    let mut complete_ips: HashSet<String> = HashSet::new();

    // Iterate over each combination of octet values and insert the IP _address into the HashSet
    for j in j_range {
        for k in k_range.clone() {
            let _ip_address= format!("10.{}.{}.{}", middle_octet, j, k);
            complete_ips.insert(_ip_address);
            
        }
    }

    //Convert Vec String to Hashmap
    let gateway_list: HashSet<String> = find_ip_or_segment_in_use.into_iter().collect();
    // Identify the missing IPs
    let available_ips: HashSet<String> = complete_ips
        .difference(&gateway_list)
        .map(|s| s.clone())
        .collect();

    let _path_available_ip  = return_path("available_IP.txt", "cache");
    //_path.push("\\data\\available_IP.txt");
    let mut file = File::create(_path_available_ip)?;
    for element in &available_ips {
        writeln!(file, "{}", element)?;
    }
    

    Ok(())
}



pub fn find_ip_or_segment_in_use(command: &String, _address: &String, username: &str, password: &str, looking_for: &str) -> io::Result<Vec<String>> {
    let mut filename = "";
    if looking_for == "dst-address=" {
        filename = "in_use_segments.txt";
    } else if looking_for == "gateway=" {
        filename = "in_use_ip.txt";
    }
    let filename = return_path(filename, "cache");


    let session = establish_ssh_connection(&_address, username, password)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("No hay conexion con el router {} \n{}", _address, err)))?;

    let channel = session.channel_session()
        .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("Channel creation error: {}", err)))?;

    let ip_in_use = send_ssh_command_save_in_txt(channel, command.to_string(), looking_for, filename)?;

    Ok(ip_in_use)
}

