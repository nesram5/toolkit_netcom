//credentials.rs
use base64;
use rpassword::read_password;
use std::process::Command;
use std::fs::{self,File};
use std::io::{self, BufReader, BufRead, Write};
use crate::auxiliar::{check_default_ro_to, return_path};
use crate::ssh::establish_ssh_connection;

pub fn option_1() -> (String, String, bool){
    // Check if Credentials.txt exists and its not empty
    let path_credentials = return_path("credentials.txt", "data");
   loop{ 
       if let Ok(metadata) = fs::metadata(path_credentials.clone()) {
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

pub fn ask_user_pass() -> io::Result<()>{
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
       //let address = "10.0.0.8:22".to_string();
       let address: String = check_default_ro_to("login".to_string()).unwrap();
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
   let path_credentials = return_path("credentials.txt", "data");
   let mut file = File::create(path_credentials)?;
   // Codificar a Base64
   
   let encoded_username = base64::encode(_username);
   writeln!(file, "{}", encoded_username)?;

   let encoded_password = base64::encode(_password);
   writeln!(file, "{}", encoded_password)?;

   Ok(())
}

pub fn decode_user_password() -> (io::Result<()> , String, String){
   let mut _vec = Vec::new();
   let path_credentials = return_path("credentials.txt", "data");
   let file = match File::open(path_credentials) {
       Ok(f) => f,
       Err(e) => panic!("Error opening file: {}", e),
   };
   let reader = BufReader::new(file);

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

pub fn already_logged() -> (String, String, bool) {
   let username:String = String::new();
   let password:String = String::new();
   let mut logged = false;
   let path_credentials = return_path("credentials.txt", "data");
   if let Ok(metadata) = fs::metadata(path_credentials) {
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