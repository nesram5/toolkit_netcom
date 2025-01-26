use crossterm::{execute, cursor, terminal};
use std::io::Write;
use std::process::Command;
use crate::io;
use std::path::PathBuf;
use std::env;

pub fn calculate_packet_loss(latencies: &Vec<f32>) -> f32 {
    // Filter out NaN values
    let valid_latencies: Vec<f32> = latencies.iter().cloned().filter(|&x| !x.is_nan()).collect();

    if valid_latencies.is_empty() {
        return 0.0; // Return 0% if there are no valid latencies
    }

    // Count the number of lost packets (latency == 9999.0)
    let lost_packets = valid_latencies.iter().filter(|&&latency| latency == 9999.0).count() as f32;
    
    let total_packets = valid_latencies.len() as f32;

    // Calculate the packet-loss percentage
    if lost_packets == total_packets {
        return 100.0;
    }

    (lost_packets * 100.0) / total_packets
}


pub fn calculate_average_latency(latencies: &Vec<f32>) -> f32 {
    // Check if the vector is empty
    if latencies.is_empty() {
        return 0.0;
    }

    // Check if there are values equal to 9999.0 and replace them with 0.0
    let mut cleaned_latencies = latencies.clone();
    for latency in &mut cleaned_latencies {
        if *latency == 9999.0 {
            *latency = 0.0;
        }
    }

    // Sum up all latency values
    let sum: f32 = cleaned_latencies.iter().sum();

    // Calculate the average latency
    let average_latency = sum / cleaned_latencies.len() as f32;

    average_latency
}

pub fn calculate_average_ttl(values: &Vec<i32>) -> i32 {
    // Check if the vector is not empty
    if values.is_empty() {
        return 0;
    }

    // Sum up all values
    let sum: i32 = values.iter().sum();

    // Calculate the average and truncate the decimal part
    let average = sum / values.len() as i32;

    average
}

pub fn find_min_max(latencies: &Vec<f32>) -> (f32 , f32) {

     // Check if there are values equal to 9999.0 and replace them with 0.0
     let mut cleaned_latencies = latencies.clone();
     for latency in &mut cleaned_latencies {
         if *latency == 9999.0 {
             *latency = 0.0;
         }
     }
       // Find the minimum and maximum values
    let min_value = *cleaned_latencies.iter().min_by(|&a, &b| a.partial_cmp(b).unwrap()).unwrap();
    let max_value = *cleaned_latencies.iter().max_by(|&a, &b| a.partial_cmp(b).unwrap()).unwrap();

    return (min_value, max_value);
}

pub fn parse_latency_value(element: &str) -> f32 {
    // Find the part of the string that contains the latency value
    let latency_part = element
        .split_whitespace()
        .find(|s| s.contains("ms") || s.contains("us"))
        .unwrap_or_default();

    // Extract milliseconds and microseconds
    if latency_part.contains("ms") && latency_part.contains("us") {
        // Split into ms and us parts
        let (ms, us) = latency_part.split_at(latency_part.find("ms").unwrap_or_default() + 2);
        let ms_value = ms.trim_end_matches("ms").parse::<f32>().unwrap_or_default();
        let us_value = us.trim_end_matches("us").parse::<f32>().unwrap_or_default() / 1000.0;
        ms_value + us_value
    } else if latency_part.contains("ms") {
        // Only milliseconds
        latency_part.trim_end_matches("ms").parse::<f32>().unwrap_or_default()
    } else if latency_part.contains("us") {
        // Only microseconds
        latency_part.trim_end_matches("us").parse::<f32>().unwrap_or_default() / 1000.0
    } else {
        // No valid latency value found
        f32::NAN
    }
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

pub fn clear_lines(start: u16, end: u16) -> io::Result<()> {
    for line in start..=end {
        execute!(io::stdout(), cursor::MoveTo(1, line), terminal::Clear(terminal::ClearType::CurrentLine))?;
    }
    Ok(())
}

pub fn clear_screen() {
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

pub fn return_path(filename: &str, data_or_cache: &str) -> PathBuf {
    let mut _path:PathBuf = env::current_dir().unwrap_or_default();
    let mut _join: PathBuf = PathBuf::new();
   if data_or_cache == "data"{
        _join = _path.join("data").join(filename);
    } else if data_or_cache == "cache" {
        _join = _path.join("cache").join(filename);
    }
    _join
}