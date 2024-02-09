from paramiko import SSHClient, AutoAddPolicy, SSHException, AuthenticationException
from braillegraph import horizontal_graph
from statistics import mean
from os import system, getcwd
from time import sleep
import netmiko 
import ipaddress

current_dir = getcwd()
routes = "{current_dir}\\routes.txt".format(current_dir = current_dir)
full_routes_ocnus = "{current_dir}\\full_routes_ocnus.txt".format(current_dir = current_dir)
ips = "{current_dir}\\ips.txt".format(current_dir = current_dir)
full_managment_ip = "{current_dir}\\full_managment_ip.txt".format(current_dir = current_dir)
managment_ip = "{current_dir}\\managment_ip.txt".format(current_dir = current_dir)
latency_avg = 0
ttl_avg = 0
hostname = '10.1.36.1'
net_connect = None
port = '22'
username = 'nramirez'
password='N3st0rR4m23*'
destination_ip = '8.8.8.8'
source_ip = '45.182.141.0'
shell = None
ip_segment_node = {
    
    "castillito":['52','10.1.52.1'],
    "castellana" : ['60', '10.1.60.1'],
    "copei" : ['32', '10.1.32.1'],
    "copei-a" : ['52', '10.1.32.1'],
    "colina" : ['40', '10.1.40.1'],
    "esmeralda" : ['56', '10.1.56.1'],    
    "flor_amarillo" : ['10.40', '10.10.40.1'],
    "guacara" : ['10.44','10.10.44.1'],
    "isla_larga": ['8','10.1.8.1'],   
    "mirador" : ['44', '10.1.44.1'], 
    "paseo" : ['10.36','10.10.36.1'],
    "parques" : ['10.48','10.10.48.1'],
    "parral" : ['36', '10.1.36.1'],
    "san_andres" : ['10.32', '10.10.32.1'],
    "torre_ejecutiva":['96','10.1.96.1'],
    "xian":['48','10.1.48.1'],
    }

def append_list_to_txt(file_path, my_list):
    with open(file_path, 'a') as file:
        for element in my_list:
            file.write(str(element) + '\n')

def command_mk_find_ip_management(segment):

    command = []
    i = 0

    if '.' in segment:
        result = segment.split(".")
        segment = int(result[1])
        while i < 4:
            segment = segment + i
            command.append('ip route print terse without-paging where gateway~"10.10.{}"'.format(segment))
            i += 1
    else:
        segment = int(segment)
        while i < 4:
            segment = segment + i
            command.append('ip route print terse without-paging where gateway~"10.1.{}"'.format(segment))
            i += 1

    return command

def connection_mikrotik_netmiko():
    global ips, net_connect
    mikrotik = {'device_type': 'mikrotik_routeros', 'host': '10.1.32.1', 'username': 'nramirez', 'password': 'N3st0rR4m23*', 'port' : 22, 'secret': 'secret', }

    net_connect = netmiko.ConnectHandler(**mikrotik)

def exec_commands_in_mk(commands):
    file = open(ips,'w')
    
    for command in commands:
        output = net_connect._send_command_timing_str(command)
        #output = net_connect.send_config_set(command)
        file.writelines(output)
        
        #file.writelines(output)
    file.close()
    
    with open(ips, 'r') as file:
        ips_list = file.readlines()
        file.close()
    return ips_list

def save_list_to_txt(file_path, my_list):
    with open(file_path, 'w') as file:
        for element in my_list:
            file.write(str(element) + '\n')

   
def print_avg_results(latency_avg, ttl_avg):
    return print("Latency: {latency_avg:,d} TTL: {ttl_avg}".format(latency_avg = latency_avg, ttl_avg = ttl_avg))

def process_ssh_terminal(stdout):
    global latency_avg, ttl_avg
    latency = []
    ttl = []
    i = 0  
    while True:

        for line in stdout:
            #avg of latency and tll
            if len(latency) > 2:
                #latency_avg = avg_of_list(latency)
                #ttl_avg = avg_of_list(ttl)
                return latency #Break
            
            #mesuare the size of the line
            string_size_in_bytes = len(line) * 2
            if string_size_in_bytes >= 175 and string_size_in_bytes <= 299: #header line
                i = i + 1
                continue
            elif string_size_in_bytes >= 160:
                latency.append(float(0))
                continue

            elif string_size_in_bytes >= 100 and string_size_in_bytes <= 159: #ping line
               
               split_output = line.split()
               ttl = split_output[3]
               ms_us = split_output[4]
               string_in_bytes = len(ms_us) * 2
               if string_in_bytes >=  18:
                    ms = re.search(r'\d+ms', ms_us)[0]#Grab Ms value
                    us = re.search(r'\d+us', ms_us)[0] #Grab Us value
                    result = (float(ms  +  us) / 1000)

               elif string_in_bytes <= 10: 
                    try:
                        ms = re.search(r'\d+ms', ms_us)#Grab Ms value
                        result = (float(ms))
                    except:
                        us = re.search(r'\d+us', ms_us) #Grab Us value
                        result = (float(us))
                    
               latency.append(result) #save value of latency for avg later                       
               i = i + 1

            if string_size_in_bytes >= 300 and string_size_in_bytes <= 399: #Mikrotik avg values
                i = i + 1
                continue

def visualizeHorizontal(stdout):
   latency = []
   while True:
        
        latency_run = process_ssh_terminal(stdout)
        if len(latency) > 100:
            latency.clear()
        if latency_run[0] != 0:

            latency.append(latency_run[0])
            latency.append(latency_run[1])
            latency.append(latency_run[2])
            latency.pop()
            maxHeightValue = 50
            for i in range(len(latency)):
                #Clear screen
                os.system('cls')
                if len(latency) > 1:
                    #Only plot if there is more than 2 values
                    maxValue = max(latency)
                    minValue = min(latency)
                    k = maxHeightValue / maxValue
                    valuelist = [int(value * k) for value in latency]
                    print('{}'.format(horizontal_graph(valuelist)))
                    print('Max: {maxValue} ms \nMin: {minValue}\nActual: {latency} \nDest. Address: {destination_ip} \nSrc-Address {source_ip}'.format(
                        maxValue = maxValue, minValue = minValue, latency = latency[-1], destination_ip = destination_ip, source_ip = source_ip))
                    
        else:
            print("Cannot connect with the destination address")

def connection_octus(command):
    global shell
    try:
        #command = "ping {destination_ip} source {source_ip}".format(destination_ip = destination_ip, source_ip = source_ip)
        shell.send(command + '\n')

        while not shell.recv_ready():
            sleep(3)

        try:
            while True:
                response = shell.recv(6625).decode()
                file = open(routes,'w')
                file.writelines(response)
                file.close()
                #response = shell.recv(1024).decode()
                print(response)
            
                if "condition" in response:
                    break
        except KeyboardInterrupt:
            pass
        
    except AuthenticationException:
        print("Authentication failed, please verify your credentials.")
    except SSHException as e:
        print("SSH connection failed:", str(e))
    except Exception as e:
        print("An error occurred:", str(e))    
    finally:
        ssh_client.close()

def read_txt_to_string(file_path):
    with open(file_path, 'r') as file:
        data = file.read().replace('\n', ' ')
    return data

def test():
    global hostname, port, username, password, destination_ip, source_ip

    ssh_client = SSHClient()
    ssh_client.set_missing_host_key_policy(AutoAddPolicy())
   

    ssh_client.connect(hostname=hostname, port=port, username=username, password=password)
        
    command = "ping {destination_ip} src-address= {source_ip}".format(destination_ip = destination_ip, source_ip = source_ip)
        
    stdin, stdout, stderr = ssh_client.exec_command(command, get_pty=True)
    while True:
        visualizeHorizontal(stdout)
    #for line in iter(stdout.readline, ""):
    #    visualizeHorizontal(line)



def test2():
    global hostname, port, username, password, destination_ip, source_ip, shell

    ssh_client = SSHClient()
    ssh_client.set_missing_host_key_policy(AutoAddPolicy())
    ssh_client.connect(hostname=hostname, port=port, username=username, password=password)
    shell = ssh_client.invoke_shell()

    
        
    #command = "ping {destination_ip} src-address= {source_ip}".format(destination_ip = destination_ip, source_ip = source_ip)

    commands= [
        "terminal length 0",
        "terminal length 0",
        "sh ip route | include 192.168.*" 
               ]
    
        #command = "ping {destination_ip} source {source_ip}".format(destination_ip = destination_ip, source_ip = source_ip)
    for command in commands:
        #while not shell.recv_ready():
        #    sleep(5)
        shell.send(command + '\r\n')
        response = shell.recv(6625).decode()
        file = open(routes,'w')
        file.writelines(response)
        file.close()
        
    
    
    
      
    #try:
        
    #    response = shell.recv(6625).decode()
    #    file = open(routes,'w')
    #    file.writelines(response)
    #    file.close()
     #   #response = shell.recv(1024).decode()
      #  #print(response)

    #except Exception as e:
    #    print("An error occurred:", str(e))    
    #finally:
    #    ssh_client.close()
    #connection_octus(command)
    #stdin, stdout, stderr = ssh_client.exec_command(command, get_pty=True)

    #save = []
    #i = 0

    #for line in iter(stdout.readline, ""):
     #  save.append(stdout.readline)
      # print(line, end="")


def connection_octus_netmiko():
    cisco_881 = {'device_type': 'cisco_ios', 'host': '10.1.2.2', 'username': 'ubnt', 'password': 'N3tc0m++', 'port' : 22, 'secret': 'secret', }

    net_connect = netmiko.ConnectHandler(**cisco_881)
    output = net_connect.send_command('terminal length 0')

    output = net_connect.send_command('sh ip route | include 192.168.*')
    file = open(routes,'w')
    file.writelines(output)
    with open(routes, 'r') as file:
        route_list = file.readlines()
    file.close()
    return route_list


def find_missing_ip_cird_29(route_list):
    global full_routes_ocnus

    i_range = range(2, 255)  # Range for the second octet
    j_range = range(0, 255, 8)  # Range for the third octet with intervals of 8
    # Generate the complete set of /29 IPs
    complete_ips = {f"192.168.{i}.{j}/29" for i in i_range for j in j_range}

    # Extract /29 IPs from the list
    existing_ips = {route.split()[2].split('[')[0] for route in route_list if '/29' in route}
    #TESTING
    save_list_to_txt(full_routes_ocnus, existing_ips)
     
    # Identify the missing /29 IPs
    missing_ips = complete_ips - existing_ips
    result = sorted(missing_ips, key=lambda ip: ipaddress.IPv4Network(ip))
    
    with open(routes, 'w') as file:
        for element in result:
            file.write(element + '\n')
    file.close()  

def get_number_before_dot(s):
    # Find the index of the dot
    dot_index = s.find('.')
    
    # Check if a dot is present in the string
    if dot_index != -1:
        # Extract the substring before the dot
        number_before_dot = s[:dot_index]
        number_before_dot = int(number_before_dot)
        return number_before_dot
    else:
        # If no dot is found, return the entire string
        return s

def find_management_ip(node_segment_initial, route_list):#return result
    global full_managment_ip, managment_ip
    
    if '.' in node_segment_initial:
        separeted = node_segment_initial.split(".")
        node_segment_initial = separeted[1]
        result = get_number_before_dot(node_segment_initial)
        node_segment_initial = int(result)
        a = 10
    else:
        node_segment_initial = int(node_segment_initial)
        a = 1

    node_segment_final = node_segment_initial + 4
    
    # Extract gateway values into a new list
    gateway_list = [entry.split("gateway=")[1].split(" ")[0] for entry in route_list]
      #TESTING
    save_list_to_txt(managment_ip, gateway_list)

    unique_gateways = list(set(gateway_list))
     #TESTING
    save_list_to_txt(full_managment_ip, unique_gateways)
  

     # Generate the complete set of IPs
    
    i_range = range(node_segment_initial, node_segment_final)  # Range for the second octet
    j_range = range(130, 255)  # Range for the third octet from 130 to 254 with intervals of 8
    

    complete_ips = {f"10.{a}.{i}.{j}" for i in i_range for j in j_range}
    file_dir = "{current_dir}\\complete_list.txt".format(current_dir = current_dir)
    save_list_to_txt(file_dir, complete_ips)

    # Identify the missing IPs
    missing_ips = set(map(str,complete_ips)) - set(map(str, unique_gateways))
    result = sorted(missing_ips, key=lambda ip: ipaddress.IPv4Network(ip))

    return result

def print_first_n_elements(file_path, n):
    with open(file_path, 'r') as file:
        lines = file.readlines()
        for i in range(min(n, len(lines))):
            print(lines[i].strip())
    
def print_ip_and_segment():
    global ips, routes
    print("Estas son 5 posibles ip para el nodo tal\n")
    print_first_n_elements(ips,5)
    print("\nEstas son 5 posibles segmentos disponibles\n")
    print_first_n_elements(routes,5)

def main():
    global ips
    connection_mikrotik_netmiko()
    commands = command_mk_find_ip_management('32')
    route_list = exec_commands_in_mk(commands)
    
    managment_ip = find_management_ip('32',route_list)
   
    save_list_to_txt(ips, managment_ip)

    route_list = connection_octus_netmiko()
    find_missing_ip_cird_29(route_list)

    print_ip_and_segment()


main()



