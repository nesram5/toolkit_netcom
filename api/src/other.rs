use std::io;

fn print_menu(ip_segment_node: &std::collections::HashMap<&str, [&str; 2]>) {
    println!("Selecciona una opción:");
    for (i, (key, value)) in ip_segment_node.iter().enumerate() {
        println!("{}. Clave: {}, Valor IP: {}", i + 1, key, value[1]);
    }
}

fn get_ip_selection(ip_segment_node: &std::collections::HashMap<&str, [&str; 2]>) -> String {
    loop {
        print_menu(&ip_segment_node);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");
        
        match input.trim().parse::<usize>() {
            Ok(selection) if selection >= 1 && selection <= ip_segment_node.len() => {
                let selected_key = ip_segment_node.keys().nth(selection - 1).unwrap();
                return ip_segment_node[selected_key][1].to_string();
            }
            _ => println!("Número fuera de rango. Inténtalo de nuevo."),
        }
    }
}

fn main() {
    let mut ip_segment_node = std::collections::HashMap::new();
    ip_segment_node.insert("castillito", ["52", "10.1.52.1"]);
    ip_segment_node.insert("castellana", ["60", "10.1.60.1"]);
    ip_segment_node.insert("copei", ["32", "10.1.32.1"]);
    // ... (Añade el resto de las entradas del diccionario)

    let selected_ip = get_ip_selection(&ip_segment_node);
    println!("La IP seleccionada es: {}", selected_ip);
}