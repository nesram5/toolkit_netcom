mod data;
use std::net::TcpStream;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::error::Error;
use ssh2::{Channel, Session};
use std::process::Command;
use std::str;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};

fn establish_ssh_connection(_address: &String,username: &str, password: &str,) -> Result<Session, Box<dyn std::error::Error>> {
    let _tcp = TcpStream::connect(_address)?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(_tcp);
    sess.handshake()?;
    // Password-based authentication
    sess.userauth_password(username, password)?;
    if !sess.authenticated() {
        return Err("Authentication failed".into());
    }
    Ok(sess)
}

fn find_ip_or_segment_in_use(command: &String, _address: &String, username: &str, password: &str, looking_for: &str) -> Result<(), std::io::Error> {
    let mut filename = "";
    if looking_for == "dst-address=" {
        filename = "in_use_segments.txt";
    } else if looking_for == "gateway=" {
        filename = "in_use_ip.txt";
    }

    let session = establish_ssh_connection(&_address, username, password)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("SSH connection error: {}", err)))?;

    let channel = session.channel_session()
        .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("Channel creation error: {}", err)))?;

    let _line = send_ssh_command_save_in_txt(channel, &command, looking_for, filename)?;

    Ok(())
}

fn clean_txt_file(filename: &str) -> io::Result<()>{
    // Check if the file exists
    let file_exists = std::path::Path::new(filename).exists();

    // Open the file in append mode or create it if it doesn't exist
    let _file = if file_exists {
        File::create(filename)?
    } else {
        File::create(filename)?
    };
    Ok(())
}

fn save_to_txt_file(data: HashSet<String>, filename: &str) -> io::Result<()> {

    let mut file = OpenOptions::new().write(true).append(true).create(true).open(filename)?;
    
    // Iterate through the data and write each element to the file
    for line in data {
        writeln!(file, "{}", line)?;
    }


    Ok(())
}

fn send_ssh_command_save_in_txt(mut channel: Channel , command: &String, looking_for: &str, filename: &str) -> io::Result<()> {
    //prueba Ocnus
    //let command_1 = "terminal length 0".to_string();
    //let _channel: Result<(), ssh2::Error> = channel.exec(&command_1);
    
    let _channel: Result<(), ssh2::Error> = channel.exec(&command);
    
    // Read the output in chunks
    let mut buffer = Vec::new();
    let mut chunk = [0; 122880]; // Adjust the chunk size as needed
    //let mut chunk = [0; 4096]; // Adjust the chunk size as needed
    let mut route_list:HashSet<String> = HashSet::new();
    
    loop {
        match channel.read(&mut chunk) {
            Ok(bytes_read) if bytes_read > 0 => {
                buffer.extend_from_slice(&chunk[..bytes_read]);

                // Check for your custom delimiter or process the data accordingly
                if let Some(delimiter_position) = buffer.iter().position(|&c| c == b'\n') {
                    // Process the data up to the delimiter (in this case, newline)
                    let output: Result<&str, str::Utf8Error> = std::str::from_utf8(&buffer[..delimiter_position]);
                    route_list.insert( format!("\n{:?}", output));

                    //REVOME THIS LATER
                    //println!("\n{:?}", output);
                     //REVOME THIS LATER

                    // Remove processed data from the buffer
                    buffer.drain(..delimiter_position + 1);
                }
            }
            Ok(_) => break,  // End of data
            Err(err) => return Err(err), // Handle errors
        }
    }
    //NOT SHOULD BE DATA REMAINING
    let remaining_output = std::str::from_utf8(&buffer).expect("Invalid UTF-8");

    // Create a BufReader around remaining_output
    let reader = io::BufReader::new(remaining_output.as_bytes());

    // Iterate over lines and insert each line into the HashSet
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        route_list.insert(line);
    }
    

    //Deleting duplicades
    let unique_list_of_ip: HashSet<String> = route_list
        .iter()
        .flat_map(|entry| entry.split(looking_for).nth(1))
        .flat_map(|gateway| gateway.split_whitespace().next())
        .filter(|&gateway| !gateway.is_empty()) // Filter out empty strings
        .map(String::from)
        .collect();

    let mut sorted_vec: Vec<String> = unique_list_of_ip.into_iter().collect();
    sorted_vec.sort(); // Sort the Vec

    // Convert the sorted Vec back to a HashSet if needed
    let _save_to_txt_file = save_to_txt_file(sorted_vec.into_iter().collect(), filename);


    Ok(())
}

fn clear_screen() {
    if cfg!(unix) {
        // For Unix-like systems (Linux, macOS)
        Command::new("clear").status().expect("Failed to clear screen");
    } else if cfg!(windows) {
        // For Windows systems
        Command::new("cmd").arg("/c").arg("cls").status().expect("Failed to clear screen");
    } else {
        // Unsupported operating system
        println!("Clear screen not supported on this platform.");
    }
}

fn print_first_five_lines(file_path: &str) -> io::Result<()> {
    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader to read the file line by line
    let reader = BufReader::new(file);

    // Use an iterator to iterate over the lines
    for (line_number, line) in reader.lines().enumerate() {
        // Check if we've printed 5 lines already
        if line_number >= 5 {
            break; // Exit the loop once we've printed 5 lines
        }

        // Print the line to the console
        println!("{}", line?);
    }

    Ok(())
}

fn ip_segment_node_choice() -> (&'static str, &'static str, &'static str){
    
    clear_screen();
    let ip_segment_node = vec![
        ("castillito", vec!["1","52", "10.1.52.1"]),
        ("castellana", vec!["1","60", "10.1.60.1"]),
        ("copei", vec!["1","32", "10.1.32.1"]),
        ("copei-a", vec!["1","52", "10.1.32.1"]),
        ("colina", vec!["1","40", "10.1.40.1"]),
        ("esmeralda", vec!["1","56", "10.1.56.1"]),
        ("flor_amarillo", vec!["10", "40", "10.10.40.1"]),
        ("guacara", vec!["10","44", "10.10.44.1"]),
        ("isla_larga", vec!["1", "8", "10.1.8.1"]),
        ("mirador", vec!["1", "44", "10.1.44.1"]),
        ("paseo", vec!["10", "36", "10.10.36.1"]),
        ("parques", vec!["10","48", "10.10.48.1"]),
        ("parral", vec!["1","36", "10.1.36.1"]),
        ("san_andres", vec!["10","32", "10.10.32.1"]),
        ("torre_ejecutiva", vec!["1","96", "10.1.96.1"]),
        ("xian", vec!["1","48", "10.1.48.1"]),
    ];
    let mut number_str: &str = "";
    let mut segment_str: &str = "";
    let mut ip_str: &str = "";
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
                number_str = number;
                segment_str = segment;
                ip_str = ip;
                
                println!("\nBuscando IP y Segmento /29 en el nodo seleccionado");
                break;
               }   
            
        } else {
            println!("Seleccion invalida");
            let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
            clear_screen();
        }
    }
   (number_str, segment_str, ip_str)
}

fn find_available_segments() -> io::Result<()> {
    

    // Generate the complete set of /29 IPs
    let complete_ip: [&str; 8096] = data::COMPLETE_IP_SEGMENTS;
    let complete_ip_segments: HashSet<String> = complete_ip
        .iter()  // Iterate over each element of the array
        .map(|&segment| segment.to_string())  // Convert &str to String
        .collect();  // Collect into a HashSet
    
    // Define the existing IPs
    let file = File::open("in_use_segments.txt")?;
    let reader = io::BufReader::new(file);

    let mut in_use_ip_segments: HashSet<String> = HashSet::new();

    for line in reader.lines() {
        let line = line?;
        in_use_ip_segments.insert(line.trim().to_string());
    }


    // Identify the missing /29 IPs
    let available_ip_segments: HashSet<String> = complete_ip_segments
        .difference(&in_use_ip_segments)
        .map(|s| s.clone())
        .collect();
    

    let mut file = File::create("available_segments.txt")?;
    for element in &available_ip_segments {
        writeln!(file, "{}", element)?;
    }

    Ok(())
}

fn find_available_management_ip( middle_octet: &str, node_segment_initial: &str) -> io::Result<()>  {
    
    let file = File::open("in_use_ip.txt")?;
    let reader = BufReader::new(file);

    let mut gateway_list: HashSet<String> = HashSet::new();

    for line in reader.lines() {
        let line = line?;
        gateway_list.insert(line.trim().to_string()); 
    }
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

    // Identify the missing IPs
    let available_ips: HashSet<String> = complete_ips
        .difference(&gateway_list)
        .map(|s| s.clone())
        .collect();

    
    let mut file = File::create("available_IP.txt")?;
    for element in &available_ips {
        writeln!(file, "{}", element)?;
    }
    

    Ok(())
}

fn option_3(username: &str, password: &str ){
    let mut _router_segment:Vec<String> = Vec::new();
    let port: &str = "22";
    let (mut _segment1, mut _segment2, mut _host) = ip_segment_node_choice();
                _router_segment.push(_segment1.to_string());
                _router_segment.push(_segment2.to_string());

    let _address = format!("{}:{}", _host, port).to_string();
    //FIND IP In Use first
    let mut looking_for: &str = "gateway=";
    let mut iterator = 0;
    while iterator <= 4 {
            let ssh_command = format!("ip route print terse without-paging where gateway~\"10.{}.{}\"\r\n",_router_segment[0], _router_segment[1]).to_string();
            iterator += 1;
            let _segment2_i32: i32 = _segment2.parse().unwrap(); // Convert the string to an integer
            let result = _segment2_i32 + iterator;
            _router_segment.insert(1, result.to_string());

            let _find_ip_or_segment_in_use = find_ip_or_segment_in_use(&ssh_command, &_address, username, password, looking_for);            
    }
    //Find available_ip
    let _find_available_management_ip = find_available_management_ip(_segment1, _segment2);

    //FIND Segments in use
    looking_for = "dst-address=";
    _host = "10.0.0.8"; //Reflector
    let _address = format!("{}:{}", _host, port).to_string();
    let ssh_command: String = "ip route print terse without-paging where dst-address~\"192.168\"\r\n".to_string();
    let _find_ip_or_segment_in_use = find_ip_or_segment_in_use(&ssh_command, &_address, username, password, looking_for);
    clear_screen();          
    //Find available_Segments
    let _find_available_segments = find_available_segments();

    //Print Results
    match print_first_five_lines("available_IP.txt"){
        Ok(_) => println!("\nAqui tienes 5 IP's disponibles\n"),
        Err(err) => eprintln!("Error: {}", err),
    }

    match print_first_five_lines("available_segments.txt"){
        Ok(_) => println!("\nAqui tienes 5 segmentos disponibles\n"),
        Err(err) => eprintln!("Error: {}", err),
    }
    //Pause for read
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

fn menu() -> std::result::Result<(), Box<dyn std::error::Error>>  {

    let username = "nramirez";
    let password = "N3st0rR4m23*";    

    println!("Bienvenido al Toolkit Netcom Nivel 1\n");
    println!("¿Qué deseas realizar?");
    println!("1) Iniciar sesión");
    println!("2) Ejecutar pruebas de ping");
    println!("3) Buscar IP y Segmento disponibles");
    println!("4) Buscar IP dinámicas en todos los nodos (próximamente)");
    println!("5) Reporte automatizado de latencias\n");

    // Loop para permitir que el usuario ingrese opciones continuamente
    loop {
        // Solicitar al usuario que ingrese una opción
        println!("Por favor, ingresa el número de la opción que deseas ejecutar:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");

        // Convertir la entrada a un número entero
        let option: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("¡Error! Debes ingresar un número.");
                continue;
            }
        };

        // Ejecutar la opción seleccionada
        match option {
            1 => println!("Iniciando sesión..."),
            2 => println!("Ejecutando pruebas de ping..."),
            3 => {
                 option_3(username, password);
            },
            4 => println!("Buscando IP dinámicas en todos los nodos (próximamente)..."),
            5 => println!("Generando reporte automatizado de latencias..."),
            _ => println!("Opción no válida. Por favor, ingresa un número del 1 al 5."),
        }

        // Salir del loop si se ha ejecutado una opción válida
        if option >= 1 && option <= 5 {
            break;
        }
    }Ok(())
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
  
    let _clean_txt_file = clean_txt_file("in_use_segments.txt");
    let _clean_txt_file = clean_txt_file("in_use_ip.txt");
    clear_screen();
    let _menu: Result<(), Box<dyn Error>> = menu();
   

    Ok(())
}