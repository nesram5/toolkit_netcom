mod data;
use std::{env, result};
use std::net::TcpStream;
use base64;
use rpassword::read_password;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::error::Error;
use ssh2::{Channel, Session};
use std::process::Command;
use std::str;
use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};

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
        .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("No hay conexion con el router {} \n{}", _address, err)))?;

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

fn read_list_ip() -> (Result<(), io::Error>, Vec<String>) {
    // Open the file
    let file = match File::open("list_ip.txt") {
        Ok(f) => f,
        Err(e) => return (Err(e), Vec::new()), // Return early if file opening fails
    };

    // Create a buffer reader to read the file line by line
    let reader = io::BufReader::new(file);

    // Create a vector to store lines
    let mut lines = Vec::new();

    // Read each line from the file and push it to the vector
    for line in reader.lines() {
        match line {
            Ok(l) => lines.push(l),
            Err(e) => return (Err(e), Vec::new()), // Return early if reading line fails
        }
    }

    // Return success and the vector of lines
    (Ok(()), lines)
}

fn save_vec_to_txt_file(data: Vec<String>, filename: &str) -> io::Result<()> {

    let mut file = OpenOptions::new().write(true).append(true).create(true).open(filename)?;
   

    // Iterate through the data and write each element to the file
    for line in data {
        writeln!(file, "{}", line)?;
    }


    Ok(())
}

fn send_ssh_command_save_in_txt(mut channel: Channel , command: &String, looking_for: &str, filename: &str) -> io::Result<()> {
    
    let _channel: Result<(), ssh2::Error> = channel.exec(&command);
    
    // Read the output in chunks
    let mut buffer = Vec::new();
    let mut chunk = [0; 122880]; // Adjust the chunk size as needed
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

    let mut sorted_vec: Vec<String> = unique_list_of_ip.clone().into_iter().collect();
    sorted_vec = sort_ip_addresses(sorted_vec); // Sort the Vec

    let _save_to_txt_file = save_vec_to_txt_file(sorted_vec , filename);
    Ok(())
}

fn sort_ip_addresses(mut ip_addresses: Vec<String>) -> Vec<String> {
    ip_addresses.sort_by(|a, b| {
        let octets_a: Vec<u8> = a.split('.').flat_map(str::parse).collect();
        let octets_b: Vec<u8> = b.split('.').flat_map(str::parse).collect();

        for (octet_a, octet_b) in octets_a.iter().zip(octets_b.iter()) {
            if octet_a != octet_b {
                return octet_a.cmp(octet_b);
            }
        }

        std::cmp::Ordering::Equal
    });

    ip_addresses
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
        ("Castillito", vec!["1","52", "10.1.52.1"]),
        ("Castellana", vec!["1","60", "10.1.60.1"]),
        ("Copei", vec!["1","32", "10.1.32.1"]),
        ("Copei-a", vec!["1","52", "10.1.32.1"]),
        ("Colina", vec!["1","40", "10.1.40.1"]),
        ("La esmeralda", vec!["1","56", "10.1.56.1"]),
        ("Flor_amarillo", vec!["10", "40", "10.10.40.1"]),
        ("Guacara", vec!["10","44", "10.10.44.1"]),
        ("Isla larga", vec!["1", "8", "10.1.8.1"]),
        ("Mirador", vec!["1", "44", "10.1.44.1"]),
        ("Paseo", vec!["10", "36", "10.10.36.1"]),
        ("Parques", vec!["10","48", "10.10.48.1"]),
        ("Parral", vec!["1","36", "10.1.36.1"]),
        ("San Andres", vec!["10","32", "10.10.32.1"]),
        ("Torre Ejecutiva", vec!["1","96", "10.1.96.1"]),
        ("Xian", vec!["1","48", "10.1.48.1"]),
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

    let mut sorted_vec: Vec<String> = available_ip_segments.clone().into_iter().collect();
    sorted_vec = sort_ip_addresses(sorted_vec); // Sort the Vec

    let mut file = File::create("available_segments.txt")?;
    for element in sorted_vec {
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

fn option_3(username:&str, password: &str ){
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

            let _find_ip_or_segment_in_use = find_ip_or_segment_in_use(&ssh_command, &_address, &username, &password, looking_for);            
    }
    //Find available_ip
    let _find_available_management_ip = find_available_management_ip(_segment1, _segment2);

    //FIND Segments in use
    looking_for = "dst-address=";
    _host = "10.0.0.8"; //Reflector
    let _address = format!("{}:{}", _host, port).to_string();
    let ssh_command: String = "ip route print terse without-paging where dst-address~\"192.168\"\r\n".to_string();
    let _find_ip_or_segment_in_use = find_ip_or_segment_in_use(&ssh_command, &_address, &username, &password, looking_for);
    clear_screen();          
    //Find available_Segments
    let _find_available_segments = find_available_segments();

    //Print Results
    match print_first_five_lines("available_IP.txt"){
        Ok(_) => println!("\nAqui tienes 5 IP's disponibles\n"),
        Err(err) => eprintln!("Error: {}", err),
    }

    fn print_first_five_segments(file_path: &str) -> io::Result<()> {
        // Open the file
        let file = File::open(file_path)?;
    
        // Create a buffered reader to read the file line by line
        let reader = BufReader::new(file);
    
        // Use an iterator to iterate over the lines
        for (line_number, line) in reader.lines().enumerate() {
            // Check if we've printed 5 lines already
            if line_number >= 382 && line_number <= 387{
                println!("{}", line?);
                 // Exit the loop once we've printed 5 lines
            } else if line_number == 388 {
                break;
            }
    
            // Print the line to the console
            
        }
    
        Ok(())
    }
    match print_first_five_segments("available_segments.txt"){
        Ok(_) => println!("\nAqui tienes 5 segmentos disponibles\n"),
        Err(err) => eprintln!("Error: {}", err),
    }
    //Pause for read
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

fn option_2(list_ip: Vec<String>, username: String, password: String){
    fn check_files_exist() -> bool {
        // Check if api.exe exists
        let api_exists = fs::metadata("api.exe").is_ok();
    
        // Check if list_ip.txt exists
        let list_ip_exists = fs::metadata("list_ip.txt").is_ok();
    
        api_exists && list_ip_exists
    }
    if check_files_exist(){
        println!("Ejecutando pruebas de ping en otra ventana");
    } else {
        println!("No detectamos el archivo api.exe, verifique su antivirus")
    }
    let commands = command_ping_test(list_ip, username, password);

    for command in &commands {
        match Command::new("powershell")
                        .arg("-Command")
                        .arg(command)
                        .output() {
            Ok(output) => {
                if output.status.success() {
                    continue;
                }
                 else {
                    eprintln!("");
                }
            }
            Err(_e) => {
                eprintln!("");
            }
        }
    }
    
}

fn command_ping_test(list_ip: Vec<String>, username: String, password: String) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();

     // Define the path to the API executable file relative to the current directory
    let api_path = {
        let mut path = env::current_dir().unwrap();
        path.push("api.exe");
        path
    };
    let api = api_path.to_str().unwrap();
    // Define the path to the WT executable file relative to the current directory
    let wt_patch: String  = api.to_string();
    let wt: String = wt_patch.replace("\\api.exe", "\\terminal\\wt.exe");


    let titletab = [
        "1-4",
        "5-8",
        "9-12",
        "13-16",
        "17-?",
    ];
    
    let mut n = 0;
    let mut i = 0;
    let j = list_ip.len() - 1;
    
    while i < j {
        if i < j && i == 0 {
            let cmd1 = format!("{} -M -w 10 --title {} {} {} {} {} {} {} {}", 
                wt, titletab[n], api, list_ip[i], list_ip[i+1], list_ip[i+2], list_ip[i+3], username, password);
            i += 4;
            commands.push(cmd1);
        } else if i < j {
            let cmd1 = format!("{} -w 10 new-tab --title {} {} {} {} {} {} {} {}", 
            wt,titletab[n], api, list_ip[i], list_ip[i+1], list_ip[i+2], list_ip[i+3], username, password);
            i += 4;
            commands.push(cmd1);
        } else {
            break;
        }

        if i < j {
            let cmd2 = format!("{} -w 10 sp --title {} -V -c {} {} {} {} {} {} {} ; mf left ",
            wt,titletab[n], api, list_ip[i], list_ip[i+1], list_ip[i+2], list_ip[i+3], username, password);
            i += 4;
            commands.push(cmd2);
        } else {
            break;
        }

        //commands.push(format!("{} -w 10 mf left",wt).to_string());

        if i < j {
            let cmd4 = format!("{} -w 10 sp --title {} -H -c {} {} {} {} {} {} {} ; mf left",
            wt,titletab[n], api, list_ip[i], list_ip[i+1], list_ip[i+2], list_ip[i+3], username, password);
            i += 4;
            commands.push(cmd4);
        } else {
            break;
        }

        commands.push(format!("{} -w 10 mf left",wt).to_string());

        if i < j {
            let cmd6 = format!("{} -w 10 sp --title {} -H -c {} {} {} {} {} {} {}",
            wt,titletab[n], api, list_ip[i], list_ip[i+1], list_ip[i+2], list_ip[i+3], username, password);
            i += 4;
            n += 1;
            commands.push(cmd6);
        } else {
            break;
        }
    }

    commands
}

fn option_1() -> (String, String, bool){
     // Check if Credentials.txt exists and its not empty
    loop{ 
        if let Ok(metadata) = fs::metadata("credentials.txt") {
            // Check if the file is not empty
            if metadata.len() > 0 {
                //"File 'credentials.txt' exists and is not empty.";
                let (_ok, username, password) = decode_user_password();
                let logged = true;
                return (username, password, logged)
            } 
        } else {
            //"File 'credentials.txt' does not exist."
            let _ask = ask_user_pass();
            continue;
        }
    }
}

fn ask_user_pass() -> io::Result<()>{
    let mut _username:String = String::new();
    let mut _password:String = String::new();

    loop {
      
        println!("Ingrese su nombre de usuario: ");
        io::stdin().read_line(&mut _username).expect("Error al leer la entrada");
        _username = _username.trim().to_string();

        // Solicita al usuario que ingrese una contraseña   
        println!("Ingrese su contraseña: ");  

        _password = read_password().unwrap();
        
        //Testing Credentials
        let address = "10.0.0.8:22".to_string();
        let _session = match establish_ssh_connection(&address, &_username, &_password) {
            Ok(_) => {
                // Connection successful, continue with some action
                break;
            }
            Err(err) => {
                println!("Credenciales invalidas");
                let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
                
                // Connection failed, handle the error
                return Err(io::Error::new(io::ErrorKind::Other, format!("Invalid credentials: {}", err)));
            }
        };
        
       
    }
    let mut file = File::create("credentials.txt")?;
    // Codificar a Base64
    
    let encoded_username = base64::encode(_username);
    println!("Texto codificado en Base64: {}", encoded_username);
    writeln!(file, "{}", encoded_username)?;

    let encoded_password = base64::encode(_password);
    println!("Texto codificado en Base64: {}", encoded_password);
    writeln!(file, "{}", encoded_password)?;

    Ok(())
}

fn decode_user_password() -> (io::Result<()> , String, String){
    let mut _vec = Vec::new();
    //let file = File::open("credentials.txt")?;

    let file = match File::open("credentials.txt") {
        Ok(f) => f,
        Err(e) => panic!("Error opening file: {}", e),
    };
    let reader = BufReader::new(file);


    // Initialize variables to hold the lines
    //let mut lines = reader.lines();

    for line in reader.lines(){
        _vec.push(line.unwrap());
    }
    
    let username = _vec.first().unwrap();
    let password = _vec.last().unwrap();
    
    // Decodificar desde Base64 (solo para demostración)
    let decoded_bytes = base64::decode(username).unwrap();
    let username = String::from_utf8(decoded_bytes).unwrap();
    

    let decoded_bytes = base64::decode(password).unwrap();
    let password  = String::from_utf8(decoded_bytes).unwrap();
   
    (Ok(()),username, password)
}

fn already_logged() -> (String, String, bool) {
    let username:String = String::new();
    let password:String = String::new();
    let mut logged = false;
    if let Ok(metadata) = fs::metadata("credentials.txt") {
        // Check if the file is not empty
        if metadata.len() > 0 {
            let (_ok, username, password) = decode_user_password();
            logged = true;
            return (username, password, logged);
        }
        else {
            return (username, password, logged);
        }
    } else{
        return (username, password, logged);
    }
 }

fn  option_4(username:String, password:String) -> Result<(), std::io::Error>{
    let ip_ftth_nodes: Vec<String> = vec![
        "10.1.51.1:22".to_string(),
        "10.1.20.1:22".to_string(),
        "10.10.44.1:22".to_string(),
        "10.1.8.1:22".to_string(),
        "10.10.48.1:22".to_string(),
        "10.10.56.1:22".to_string(),
        "10.10.36.1:22".to_string(),
        "10.1.50.1:22".to_string(),
        "10.0.3.2:22".to_string(),
        "10.1.44.1:22".to_string()];
    let ip_ffth_nodes_names: Vec<&str> = vec![
        "FM",
        "CEC",
        "GU", 
        "IL", 
        "LP", 
        "MS", 
        "PLI",
        "SLA",
        "TE", 
        "MRD"
    ];
    
    let filename = "dynamic_ips.txt";
    let command:String = "ip dhcp-server lease print terse without-paging where dynamic".to_string();
    let mut iterator = 0;
    for address in ip_ftth_nodes{
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("dynamic_ips.txt")?;
            writeln!(file, "Ip dinamicas del nodo: {} {}\n",ip_ffth_nodes_names[iterator],address)?;
            

            let session = establish_ssh_connection(&address, &username, &password)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("No hay conexion con el router {} \n{}", &address, err)))?;

            let channel = session.channel_session()
                .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("Channel creation error: {}", err)))?;

            let looking_for = "address=";
            let _line = send_ssh_command_save_in_txt(channel, &command, looking_for, filename)?;
            writeln!(file, "-----------------------------\n")?;
            iterator+=1;
            if iterator == 9{
                break;
        }
    }
    //Print Results
    let file = match File::open("dynamic_ips.txt") {
        Ok(f) => f,
        Err(e) => panic!("Error opening file: {}", e),
    };
    let reader = BufReader::new(file);
    clear_screen();
    println!("\t\tIp Dinamicas");
    let mut result:Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        result.push(line.trim().to_string());
        println!("{}",result.last().unwrap().as_str());
    }

    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
    Ok(())
}

fn menu() -> std::result::Result<(), Box<dyn std::error::Error>>  {
    let (_ok, list_ip) = read_list_ip();
        
    let (mut username, mut password, mut logged) = already_logged();

    let mut _iterator = 0;
    loop {
        println!("###############################################################################");
        println!("#                                                                             #");
        println!("#                     Bienvenido al Toolkit Netcom Nivel 1                    #") ;
        
        println!("#\t¿Qué deseas realizar?                                                 #");
        if logged{
            println!("#\t1) Sesion iniciada como {} ¿Cerrar sesion?", username);
        } else {
            println!("#\t1) Iniciar sesión                                                     #");
        }
        println!("#\t2) Ejecutar pruebas de ping                                           #");
        
        println!("#\t3) Buscar IP y Segmento disponibles                                   #");
        
        println!("#\t4) Buscar IP dinámicas en todos los nodos                             #");
        
        println!("#\t5) Reporte automatizado de latencias                                  #");
        println!("#\t6) Cerrar                                                             #");
        println!("#                                                                             #");
        println!("###############################################################################");
        println!("                                                  Programado por Nestor Ramirez\n");
        println!("\n");

        // Loop para permitir que el usuario ingrese opciones continuamente
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
            1 => { //Iniciar sesión
                if logged{
                    //LogOut delete "credentials.txt"
                    logged = false;
                    let file = "credentials.txt";
                    match fs::remove_file(file) {
                        Ok(()) => println!("File '{}' successfully deleted.", file),
                        Err(e) => println!("Error deleting file '{}': {}", file, e),
                    }
                }else{
                    //Iniciar sesión
                    (username, password, logged) = option_1();
                }
            },

            2 => {//Ejecutar pruebas de ping  
                if logged{
                    option_2(list_ip.clone(), username.clone(), password.clone());
                }else {
                    println!("Debes iniciar sesion, para ejecutar esta función")
                }
                    
            },
            
            3 => {//Buscar IP y Segmento disponibles 
                if logged{
                    option_3(&username, &password);
                    
                }else {
                    println!("Debes iniciar sesion, para ejecutar esta función")
                }
            },
            4 => { //"Buscando IP dinámicas en todos los nodos (próximamente)..."),
                if logged{
                    println!("Buscando IP dinámicas en todos los nodos");
                    let _option = option_4(username.clone(), password.clone());
                    
                }else {
                    println!("Debes iniciar sesion, para ejecutar esta función")
                }
            },

            5 => println!("Generando reporte automatizado de latencias..."),
            6 => //Cerrar
                break,
            _ => println!("Opción no válida. Por favor, ingresa un número del 1 al 5."),
        }

        // Salir del loop si se ha ejecutado una opción válida
        if option >= 1 && option <= 6 {
            continue;
            }
            
        
    }Ok(())
}
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
  
    let _clean_txt_file = clean_txt_file("in_use_segments.txt");
    let _clean_txt_file = clean_txt_file("in_use_ip.txt");
    let _clean_txt_file = clean_txt_file("dynamic_ips.txt");
    clear_screen();
    let _menu: Result<(), Box<dyn Error>> = menu();
   

    Ok(())
}