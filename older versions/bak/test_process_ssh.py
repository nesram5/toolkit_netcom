import re
import os
import time
from paramiko import SSHClient, AutoAddPolicy, SSHException, AuthenticationException
from braillegraph import horizontal_graph, vertical_graph
from statistics import mean

latency_avg = []
ttl = []
packet_loss_percentage = 0
hostname = '10.0.1.20'
port = '22'
username = 'nramirez'
password='N3st0rR4m23*'
destination_ip = '172.16.2.78'
source_ip = '172.16.2.77'


def escape_ansi(line):
    ansi_escape = re.compile(r'(\x9B|\x1B\[)[0-?]*[ -\/]*[@-~]')
    line_without_ansi = ansi_escape.sub('', line)
    
    # Remove additional characters
    line_without_extra_chars = line_without_ansi.replace('\f', '').replace('\r', '').replace('\t', '').replace('\n', '')
    
    return line_without_extra_chars
    
                                 

def avg_of_list(list):

    total = sum(list)
    average = total / len(list)
    return average


def print_avg_results(latency_avg, ttl_avg):
    return print("Latency: {latency_avg:,d} TTL: {ttl_avg}".format(latency_avg = latency_avg, ttl_avg = ttl_avg))

def process_ssh_terminal(stdout):
    global latency_avg, ttl, packet_loss_percentage
    latency = []
    #Add 0 value to average
    if len(latency_avg) <= 0:
        latency_avg.append(0)

    combined_value = 0
    i = 0  

    while True:

        for line in iter(stdout.readline, ""):
            clean_line = escape_ansi(line)
            line = clean_line
            # Header line
            if 'SEQ HOST' in line:
                continue

            # Averages line
            elif 'sent=' in line and 'received=' in line and 'packet-loss=' in line:
                elements = line.split()

                for element in elements:
                    if 'packet-loss=' in element:
                        packet_loss_percentage = float(element.split('=')[1].strip('%'))
                    elif 'avg-rtt=' in element:
                        if 'ms' in element and 'us' in element:
                        # Extract values for ms and us, and combine them into a float
                            element = element.split()
                            if len(element) >= 5:
                                value = element[4]
                                ms, us = value.split('ms')
                                combined_value = float(ms) + float(us.rstrip('us')) / 1000.0
                        elif 'ms' in element:
                            element = element.split()
                            if len(element) >= 5:
                                value = element[4]
                                # Save ms value to ms_list
                                combined_value = float(value.rstrip('ms'))
                        elif 'us' in element:
                            element = element.split()
                            if len(element) >= 5:
                                value = element[4]
                                # Save us value to us_list
                                combined_value = float(value.rstrip('us'))
                    #Result avg_rtt
                    latency_avg.append(combined_value)
                    
            
            # Ping value line
            elif line.strip().endswith(('ms', 'us')):
                
                
                if 'ms' in line and 'us' in line:
                # Extract values for ms and us, and combine them into a float
                    line = line.split()
                    if len(line) >= 5:
                        value = line[4]
                        ms, us = value.split('ms')
                        combined_value = float(ms) + float(us.rstrip('us')) / 1000.0
                        latency.append(combined_value)
                elif 'ms' in line:
                    line = line.split()
                    if len(line) >= 5:
                        value = line[4]
                        # Save ms value to ms_list
                        combined_value = float(value.rstrip('ms'))#changed line for value
                        latency.append(combined_value)
                elif 'us' in line:
                    line = line.split()
                    if len(line) >= 5:
                        value = line[4]
                        # Save us value to us_list
                        combined_value = float(value.rstrip('us'))#changed line for value
                        latency.append(combined_value)
                #Grab TLL value
                
                if len(line) >= 5:
                    ttl.append(int(line[3]))

                
        
            elif 'could not...' in line:
                print("Could not socket\n")
                print("#########           ERROR           #######\n")
                
            elif 'packet-loss=100%' in line:
                for element in elements:
                    if 'packet-loss=' in element:
                        packet_loss_percentage = float(element.split('=')[1].strip('%'))
                print("Could not socket\n")
                print("#########           ERROR           #######\n")
                
            else:
                print("Unknown Line:")
                print(line)

            #avg of latency and tll
            if len(latency) > 2:
                #latency_avg = avg_of_list(latency)
                #ttl_avg = avg_of_list(ttl)
                return latency #Break

def visualizeHorizontal(stdout):
   global latency_avg, ttl
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
                    print('Max: {maxValue} ms \nMin: {minValue} ms \nActual: {latency} ms \nAVG: {avg} ms \nAVG TTL: {avg_ttl} \nDest. Address: {destination_ip} \nSrc-Address {source_ip}'.format(
                        maxValue = maxValue, minValue = minValue, avg = latency_avg[-1], avg_ttl = avg_of_list(ttl), latency = latency[-1], destination_ip = destination_ip, source_ip = source_ip))
                    
                    
        else:
            #Latency value is 0
            latency.append(latency_run[0]+ 0.1)
            latency.append(latency_run[1]+ 0.1)
            latency.append(latency_run[2]+ 0.1)
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
                    print('Max: {maxValue} ms \nMin: {minValue} ms \nActual: {latency} ms \nAVG: {avg} ms \nAVG TTL: {avg_ttl} \nDest. Address: {destination_ip} \nSrc-Address {source_ip}'.format(
                        maxValue = maxValue, minValue = minValue, avg = latency_avg[-1], avg_ttl = avg_of_list(ttl), latency = latency[-1], destination_ip = destination_ip, source_ip = source_ip))



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
    global hostname, port, username, password, destination_ip, source_ip

    ssh_client = SSHClient()
    ssh_client.set_missing_host_key_policy(AutoAddPolicy())
   

    ssh_client.connect(hostname=hostname, port=port, username=username, password=password)
        
    command = "ping {destination_ip} src-address= {source_ip}".format(destination_ip = destination_ip, source_ip = source_ip)
        
    stdin, stdout, stderr = ssh_client.exec_command(command, get_pty=True)

    save = []
    i = 0

    for line in iter(stdout.readline, ""):
       save.append(stdout.readline)
       print(line, end="")

test()