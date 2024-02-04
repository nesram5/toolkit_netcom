ip_segment_node = {
    "castillito": ['52', '10.1.52.1'],
    "castellana": ['60', '10.1.60.1'],
    "copei": ['32', '10.1.32.1'],
    "copei-a": ['52', '10.1.32.1'],
    "colina": ['40', '10.1.40.1'],
    "esmeralda": ['56', '10.1.56.1'],
    "flor_amarillo": ['10.40', '10.10.40.1'],
    "guacara": ['10.44', '10.10.44.1'],
    "isla_larga": ['8', '10.1.8.1'],
    "mirador": ['44', '10.1.44.1'],
    "paseo": ['10.36', '10.10.36.1'],
    "parques": ['10.48', '10.10.48.1'],
    "parral": ['36', '10.1.36.1'],
    "san_andres": ['10.32', '10.10.32.1'],
    "torre_ejecutiva": ['96', '10.1.96.1'],
    "xian": ['48', '10.1.48.1'],
}

def print_menu():
    print("Selecciona una opción:")
    for i, (key, value) in enumerate(ip_segment_node.items(), start=1):
        print(f"{i}. {key}, IP: {value[1]}")

def get_ip_selection():
    while True:
        print_menu()
        try:
            selection = int(input("Ingresa el número de la opción deseada: "))
            if 1 <= selection <= len(ip_segment_node):
                selected_key = list(ip_segment_node.keys())[selection - 1]
                return ip_segment_node[selected_key][1]
            else:
                print("Número fuera de rango. Inténtalo de nuevo.")
        except ValueError:
            print("Por favor, ingresa un número válido.")

# Obtener la IP seleccionada
selected_ip = get_ip_selection()
print(f"La IP seleccionada es: {selected_ip}")
