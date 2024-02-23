//reports.rs
use std::thread;
use std::fs::{self,File};
use std::process::Command;
use std::time::{Duration, Instant};
use serde_json::Value;
use linked_hash_map::LinkedHashMap;
use chrono::Local;
use std::io::{self, BufRead, BufReader, Write};
use crate::ssh::command_ping_test_or_report;
use crate::auxiliar::{return_path, clear_screen, save_command_to_bat, read_file_to_vec};
use crossterm::{execute, cursor, terminal};
use serde_json::json;
use std::process;
const BANNER_LINE: u16 = 1;
//const PING_RESULTS_START_LINE: u16 = 1;
//const PING_RESULTS_END_LINE: u16 = 16;


pub fn option_5(_list_ip: Vec<String>, _username: &str, _password: &str) -> io::Result<()> {
    /*
    Medir el tamaño de list ip(total / 4) con el de report_template(omitir las lineas # contar solo $)
    sino son iguales los resultados, las pruebas extras no se tomaran en cuenta en el reporte (restar de list_ip_vec)
    Genera un archivo .bat que ejecuta API.exe en modo reporte
    Ejecutar con Api en modo reporte las pruebas de ping
    Leer cada archivo en "title" y añadirlo line0 a report_lat y line1 a repor_pl
    Generar fecha y Hora
    Desplegar el reporte
    */
    let mut name_of_tests:Vec<String> = Vec::new();
    let _commands:Vec<String> = Vec::new();    
    let path_commands = return_path("commands.bat", "cache");
    match check_total_test_for_report(_list_ip) {
        Ok((_list_ip_for_report, _name_of_tests)) => {
            // Handle the success case here
            name_of_tests = _name_of_tests.clone();
            match command_ping_test_or_report(_list_ip_for_report.clone(), _username.to_string(), _password.to_string(), 2) {
                Ok(_commands) => {
                    let _save_to_txt_file =  save_command_to_bat(_commands, path_commands.clone());
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }

    let _status = Command::new("cmd")
        .args(&["/C", path_commands.to_str().unwrap_or_default()])
        .status()
        .expect("Failed to execute bat file");

   //Pause execution of the programs for wait the results
    clear_screen();
    let duration = Duration::from_secs(120); // Set the duration for the countdown (60 seconds)
    let start_time = Instant::now(); // Get the current time

    while start_time.elapsed() < duration {
        let remaining_time = duration - start_time.elapsed();
        
        let banner_text = format!("\tEspera un dos minutos mientras compilamos el reporte \nLos calculos son hechos en base a una muestra de 100 paquetes \n\nTiempo restante para el reporte: {}", remaining_time.as_secs());
        if banner_text.contains("99"){
            clear_screen();
        }
        if banner_text.contains("9"){
            clear_screen();
        }
        print_line(&banner_text, BANNER_LINE)?;
        // Sleep for 1 second
        thread::sleep(Duration::from_secs(1));
    }
    //Read results of latency_avg and Paclet_lost from the files created by api.exe report mode
    
    let mut report_latency_avg: String = String::new();
    let mut report_packet_loss: String = String::new();

    let _copy_name_of_tests = name_of_tests.clone();

    let mut map_report: LinkedHashMap<String, Value> = LinkedHashMap::new();
    //Read the results from files on cache folder
    for element  in name_of_tests.clone() {
       
        let filename = return_path(&element, "cache");
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        // Leer las primeras dos líneas de cada archivo
        let mut lines = reader.lines();

        // Agregar la primera línea al vector report_lat
        if let Some(Ok(line)) = lines.next() {
            report_latency_avg = line;
        }

        // Agregar la segunda línea al vector report_pl
        if let Some(Ok(line)) = lines.next() {
            report_packet_loss = line;
        }
        
        map_report.entry(element.to_string()).or_insert(json!([report_latency_avg, report_packet_loss]));
    }
    //Clean
    remove_cache_file(name_of_tests.clone());
    //Make the report
    let _res = make_the_report(map_report, name_of_tests.clone());

    Ok(())
}

fn make_the_report(map_report : LinkedHashMap<String, Value>, name_of_tests: Vec<String>) -> io::Result<()> {
    // Read the content of the input file
    let current_date = Local::now().format("%d/%m/%Y").to_string();
    let current_time = Local::now().format("%I:%M %p").to_string().to_lowercase();
    let current_date_to_file = Local::now().format("%d-%m-%Y").to_string();
    let current_time_to_file = Local::now().format("%I-%M-%p").to_string().to_lowercase();

    let output_file_name = format!("Reporte{}_{}.txt",current_date_to_file,current_time_to_file).to_string();
    
    let path_report_template = return_path("report_template.txt", "data");
        
    let file = File::open(path_report_template.clone())?;
    let reader_report_template = io::BufReader::new(file);
    let report_template_vec: Vec<String> = reader_report_template.lines().map(|line| line.unwrap()).collect();
    
    let mut replaced_text = replace_text(report_template_vec, map_report.clone(), current_date, current_time, name_of_tests);
    
    replaced_text.sort_by_key(|line| {
        line.split_whitespace()
            .next()
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap_or(usize::MAX)
    });
    

   
    let _replaced_text_clean: Vec<String> = remove_chars_from_vec(replaced_text, &['[', ']', '"', ',']);

    let mut final_report:Vec<String> = Vec::new();
    for mut element in _replaced_text_clean{
        element.remove(0);
        element.remove(0);
        final_report.push(element);
    }
    // Write the modified content to the output file
    let path_output_file = return_path(&output_file_name, "cache");
    let mut output_file = File::create(path_output_file.clone())?;
    let modified_content_string = final_report.join("\n");
    output_file.write_all(modified_content_string.as_bytes())?;
    output_file.flush()?; // Ensure all data is written before closing
    output_file.sync_all()?; // Ensure all data is written to disk before closing
    drop(output_file);
   
    //Open report with notepad.exe
    clear_screen();
    println!("Abiendo el reporte en Bloc de Notas");
    let _ = Command::new("cmd.exe").arg("/c").arg("notepad.exe").arg(path_output_file).status();
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
    Ok(())
}

fn check_total_test_for_report(mut _list_ip: Vec<String>) -> io::Result<(Vec<String>,Vec<String>)> {
    let path_report_template = return_path("report_template.txt", "data");
    let _report_template = read_file_to_vec(path_report_template)?;
    let mut _name_of_tests:Vec<String> = Vec::new();

    // Iterate over chunks of size 4
    for chunk in _list_ip.chunks(4) {
        // Get the first value of each chunk
        if let Some(first_value) = chunk.first() {      
            _name_of_tests.push((*first_value.clone()).to_string());
        }
    }
    let _check_extra_test = _name_of_tests.len() - _report_template.len();

    if _check_extra_test > 0{
        let mut _i = 0;
        loop{
            _list_ip.truncate(_list_ip.len().wrapping_sub(4));
            _name_of_tests.pop();
            _i += 1;
            if _i == _check_extra_test{
                break;
            }
        }
        return Ok((_list_ip, _name_of_tests))
    }
    else {
        
        return Ok((_list_ip, _name_of_tests))
    }

}

fn remove_cache_file(name_of_tests: Vec<String>){
    
    let _file_path = return_path("commands.bat", "cache") ;
    let _file_path_str = _file_path.to_str().unwrap_or_default();
    match fs::remove_file(_file_path.clone()) {
      Ok(_) => println!(""),
       Err(err) => eprintln!("Failed to delete file '{}': {}", _file_path_str , err),
    }
   for element in  name_of_tests{
    let path_element = return_path(&element, "cache");
    let path_element_str = path_element.to_str().unwrap_or_default();
    match fs::remove_file(path_element.clone()) {
        Ok(_) => println!(""),
        Err(err) => eprintln!("Failed to delete file '{}': {}", path_element_str, err),
        } 
   }    
}

fn replace_text(mut original_text: Vec<String>, map: LinkedHashMap<String, Value>, current_date: String, current_time: String, name_of_test: Vec<String>) -> Vec<String>  {
    //let mut replaced_text: HashSet<String> = HashSet::new();
    let mut replaced_text: Vec<String> = Vec::new();
    let mut _index = 0;
    let len_original_text = original_text.len();

    loop {
        if replaced_text.len() == len_original_text{
            break;
        }
        else {
            _index = 0;
        }
    for element  in &name_of_test {
        if replaced_text.len() == len_original_text{
            break;
        }
        else {
            _index = 0;
        }
        while _index < original_text.len() {
            let line = &original_text[_index];

            if line.contains(element) {
                let values = map.get(element).unwrap();
                let line_mod = line.replace(element, &values.to_string() ).replace("$", "");
                //replaced_text.insert(line_mod);
                replaced_text.push(line_mod);
                original_text.remove(_index);
                

            } else if line.contains("_date_") && line.contains("_hour_am_pm_") {
                let line_mod = line.replace("_date_", &current_date).replace("_hour_am_pm_", &current_time).replace("#", "");
                //replaced_text.insert(line_mod);
                replaced_text.push(line_mod);
                original_text.remove(_index);
            } else if line.contains("#") {
                let line_mod = line.replace("#", "");
                //replaced_text.insert(line_mod);
                replaced_text.push(line_mod);
                original_text.remove(_index);
            }
            else if line.contains("Invalid ar...") {
                process::exit(0);
                
            }

            _index += 1;
                        

        }
    }
    }
    replaced_text
}

fn remove_chars_from_vec(vec: Vec<String>, chars: &[char]) -> Vec<String> {
    vec.into_iter()
        .map(|mut s| {
            for &c in chars {
                s.retain(|x| x != c);
            }
            s
        })
        .collect()
}

pub fn print_line(content: &str, line: u16) -> io::Result<()> {
    execute!(
        io::stdout(),
        //cursor::MoveTo(1, line),
        cursor::MoveTo(0, line),
        terminal::Clear(terminal::ClearType::CurrentLine)
    )?;
    println!("{}", content);
    io::stdout().flush().unwrap();
    Ok(())
}