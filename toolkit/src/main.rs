mod data;
mod reports;
mod find_ip;
mod find_dynamic;
mod run_tests;
mod backup;
mod ssh;
mod credentials;
mod auxiliar;
use std::env;
use std::path::PathBuf;
use std::io::{self, Write};
use std::error::Error;
use std::process::{self,Command};
use std::str;
use std::fs::{self, OpenOptions};
use crate::auxiliar::{clear_screen, return_clean_vec_list_ip, return_path};
use crate::credentials::{option_1, already_logged};
use crate::run_tests::option_2;
use crate::find_ip::option_3;
use crate::find_dynamic::option_4;
use crate::reports::option_5;

fn menu() -> std::result::Result<(), Box<dyn std::error::Error>>  {
    
    let path_ping_test_list_ip = return_path("ping_test_list_ip.json", "data");
    let ping_test_list_ip: Vec<String> = return_clean_vec_list_ip(path_ping_test_list_ip)?;
        
    let (mut username, mut password, mut logged) = already_logged();

    let mut _iterator = 0;
    loop {
        clear_screen();
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
        
        println!("#\t4) Buscar IP dinámicas en todos los nodos FTTH                        #");
        
        println!("#\t5) Reporte de latencias de Proveedores y TD                           #");
        println!("#\t6) Reporte de baterias y monitor electrico (Poximamente)              #");
        println!("#\t7) Cerrar                                                             #");
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
                    let path_credentials = return_path("credentials.txt", "data");
                    let path_credential_str = path_credentials.to_str().unwrap_or_default();
                    match fs::remove_file(path_credentials.clone()) {
                        Ok(()) => {
                            clear_screen();
                            println!("Credenciales borradas");
                            let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
                            },
                        Err(e) => println!("Error deleting file '{}': {}", path_credential_str , e),
                    }
                }else{
                    //Iniciar sesión
                    (username, password, logged) = option_1();
                    clear_screen();
                }
            },

            2 => {//Ejecutar pruebas de ping  
                if logged{
                   option_2(ping_test_list_ip.clone(), username.clone(), password.clone());
                }else {
                    clear_screen();
                    println!("Debes iniciar sesion, para ejecutar esta función");
                    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
                }
                    
            },
            
            3 => {//Buscar IP y Segmento disponibles 
                if logged{
                    option_3(&username, &password);
                    
                }else {
                    clear_screen();
                    println!("Debes iniciar sesion, para ejecutar esta función");
                    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
                }
            },
            4 => { //"Buscando IP dinámicas en todos los nodos (próximamente)..."),
                if logged{
                    println!("Buscando IP dinámicas en todos los nodos");
                    let _option = option_4(username.clone(), password.clone());
                    
                }else {
                    clear_screen();
                    println!("Debes iniciar sesion, para ejecutar esta función");
                    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
                }
            },

            5 => {
                if logged{
                    println!("Generando reporte de latencias automatizado");
                    let _option = option_5(ping_test_list_ip.clone(), &username, &password);
                    
                }else {
                    clear_screen();
                    println!("Debes iniciar sesion, para ejecutar esta función");
                    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
                }
            },
            6 => {
                println!("Aun no disponible");
                let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
            },
            7 => //Cerrar
                break,
            _ => println!("Opción no válida. Por favor, ingresa un número del 1 al 7."),
        }

        // Salir del loop si se ha ejecutado una opción válida
        if option >= 1 && option <= 7 {
            continue;
            }
            
        
    }Ok(())
}

fn check_files_exist() -> io::Result<()> {
    let mut result:String = String::new();
    //check /data
    let data_dir = return_path("", "data");
    if !data_dir.exists() {
        fs::create_dir(data_dir)?;
    }   
    //check /cache
    let cache_dir = return_path("", "cache");
    if !cache_dir.exists() {
        // Create the .ssh directory if it doesn't exist
        fs::create_dir(cache_dir)?;
    } 
    // Check if api.exe exists
    if !fs::metadata("api.exe").is_ok(){
        result.push_str("No se detecto el archivo api.exe, verifique su antivirus \n");
    }

    //check if wt.exe exists
    let mut _path:PathBuf = env::current_dir().unwrap_or_default();
    let path_wt = _path.join("terminal").join("wt.exe");
    if !fs::metadata(path_wt).is_ok(){
        result.push_str("No se detecto el archivo /terminal/wt.exe, verifique su antivirus o vuelva a descargar el archivo de nuevo\n");
    }
    
        fn save_var_to_file(data: &str, filename: PathBuf) -> io::Result<()> {

            let mut file = OpenOptions::new().write(true).append(true).create(true).open(filename)?;
        
            file.write_all(data.as_bytes())?;
        
            Ok(())
        }
    //check ping_test_list_ip
    let path_ping_test_list_ip = return_path("ping_test_list_ip.json", "data");
    if !path_ping_test_list_ip.exists() {
        let ping_test_list_ip: &str = backup::backup_data("ping_test_list_ip");
        
            match save_var_to_file(ping_test_list_ip, path_ping_test_list_ip.clone()) {
                Ok(())=> {println!("Se restauro exitosamente el archivo /data/ping_test_list_ip.json")},
                Err(err)=>{eprintln!("{}",err)}
            };
    }
    //check report_template
    let path_report_template = return_path("report_template.txt", "data");
    if !path_report_template.exists() {
        let report_template: &str = backup::backup_data("report_template");
        
            match save_var_to_file(report_template, path_report_template.clone()) {
                Ok(())=> {println!("Se restauro exitosamente el archivo /data/report_template.txt")},
                Err(err)=>{eprintln!("{}",err)}
            };
    }
    //check ftth_nodes
    let path_ftth_nodes = return_path("ftth_nodes.json", "data");
    if !path_ftth_nodes.exists() {
        let ftth_nodes: &str = backup::backup_data("ftth_nodes");
        
            match save_var_to_file(ftth_nodes, path_ftth_nodes.clone()) {
                Ok(())=> {println!("Se restauro exitosamente el archivo /data/ftth_nodes.json")},
                Err(err)=>{eprintln!("{}",err)}
            };
    }
    //check rf_nodes
    let path_rf_nodes = return_path("rf_nodes.json", "data");
    if !path_rf_nodes.exists() {
        let rf_nodes: &str = backup::backup_data("rf_nodes");
        
            match save_var_to_file(rf_nodes, path_rf_nodes.clone()) {
                Ok(())=> {println!("Se restauro exitosamente el archivo /data/rf_nodes.json")},
                Err(err)=>{eprintln!("{}",err)}
            };
    }
    //check default_config
    let path_default_config = return_path("default_config.json", "data");
    if !path_default_config.exists() {
        let default_config: &str = backup::backup_data("default_config");
        
            match save_var_to_file(default_config, path_default_config.clone()) {
                Ok(())=> {println!("Se restauro exitosamente el archivo /data/default_config.json")},
                Err(err)=>{eprintln!("{}",err)}
            };
    }
    if !result.is_empty() {
        println!("{}",result);
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
        process::exit(1);
    };

    Ok(())
} 
   

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
  match check_files_exist(){
    Ok(()) => {print!("");},
    Err(e) => {eprint!("{}",e)}
  };
    
    clear_screen();
    let _menu: Result<(), Box<dyn Error>> = menu();
   

    Ok(())
}

