from re import compile
from os import system
from sys import argv
from paramiko import SSHClient, AutoAddPolicy, SSHException, AuthenticationException
from braillegraph import horizontal_graph

latency_avg = []
latency = []
ttl = []
packet_loss_percentage = 0

def escape_ansi(line):
    ansi_escape = compile(r'(\x9B|\x1B\[)[0-?]*[ -\/]*[@-~]')
    line_without_ansi = ansi_escape.sub('', line)
    
    # Remove additional characters
    line_without_extra_chars = line_without_ansi.replace('\f', '').replace('\r', '').replace('\t', '').replace('\n', '')
    
    return line_without_extra_chars

def avg_of_list(list):

    total = sum(list)
    average = total / len(list)
    return average

def process_ssh_terminal(stdout):
    global latency_avg, ttl, packet_loss_percentage, latency
    
    #Add 0 value to average
    if len(latency_avg) == 0:
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
                                combined_value = float(value.rstrip('us')) / 1000.0
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
                        combined_value = float(value.rstrip('us')) / 1000.0#changed line for value
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
                
            elif 'timeout' in line:
                print("Could not socket\n")
                print("#########           ERROR           #######\n")
                
            else:
                print("Unknown Line:")
                print(line)
            i = i + 1
            if i == 3:
                return True

def visualizeHorizontal(stdout, title, destination_ip, source_ip):
   global latency_avg, ttl, packet_loss_percentage, latency
   while True:
        
        process_ssh_terminal(stdout)
        

        if latency[-1] != 0:
            a = 1
                    
        elif latency[-1] == 0:
            #Latency is equal to zero
            latency[-1] = latency[-1]+ 0.1
            #latency.append(latency_run[1]+ 0.1)
            #latency.append(latency_run[2]+ 0.1)
                   
        else:
            print("Cannot connect with host {}".format(title))
            input()
            break
        
        #latency.pop()
        maxHeightValue = 25
        for i in range(len(latency)):
                #Clear screen
                system('cls')
                if len(latency) > 1:
                   
                    #Only plot if there is more than 2 values
                    maxValue = max(latency)
                    minValue = min(latency)
                    k = maxHeightValue / maxValue
                    valuelist = [int(value * k) for value in latency]
                    print('{}'.format(horizontal_graph(valuelist)))
                    print("\n{title}" .format(title = title)) 
                    print('\nMax:{maxValue} ms \nMin: {minValue} ms \nActual: {latency} ms \nAVG: {avg} ms \nAVG TTL: {avg_ttl} \nPackage Lost: {package} % \nDest. Address: {destination_ip} \nSrc-Address {source_ip}' .format(
                        maxValue = maxValue, minValue = minValue, avg = latency_avg[-1], avg_ttl = avg_of_list(ttl), package = packet_loss_percentage, latency = latency[-1], destination_ip = destination_ip, source_ip = source_ip))
                    if len(latency_avg) >= 2:
                        try:
                            latency_avg.remove(0)
                        except:
                            continue
        if len(latency) > 30:
            save = latency[-1]
            latency.clear()
            latency.append(save)
                            
def connection_mikrotik(title, destination_ip, source_ip, host, usern, passw):
    
    port = 22
    ssh_client = SSHClient()
    ssh_client.set_missing_host_key_policy(AutoAddPolicy())

    try:

        # Enable logging to a file
        ssh_client.connect(hostname=host, port=port, username=usern, password=passw)
        
        command = "ping {destination_ip} src-address= {source_ip}".format(destination_ip = destination_ip, source_ip = source_ip)
        
        stdin, stdout, stderr = ssh_client.exec_command(command, get_pty=True)

        while True:
            visualizeHorizontal(stdout, title, destination_ip, source_ip)
    except AuthenticationException:
        print("Authentication failed, please verify your credentials.")
        input()
    except SSHException as e:
        print("SSH connection failed: trying to connect with {}\n".format(title), str(e))
        input()
    except Exception as e:
        print("An error occurred trying to connect with {}\n".format(title), str(e))
        input()
    finally:
        ssh_client.close()

def connection_octus(title,destination_ip,source_ip,hostname,username, password):
    
    port = 22
    ssh_client = SSHClient()
    ssh_client.set_missing_host_key_policy(AutoAddPolicy())

    try:
        ssh_client.connect(hostname=hostname, port=port, username=username, password=password)
        shell = ssh_client.invoke_shell()
        command = "ping {destination_ip} source {source_ip}".format(destination_ip = destination_ip, source_ip = source_ip)
        shell.send(command + '\n')

        #while not shell.recv_ready():
        #    sleep(3)

        try:
            while True:
                response = shell.recv(1024).decode()
                print(response)
                print("{title}".format(title = title))
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
        
def wich_model(title, destination_ip, source_ip, hostname, username, password):
    
    if hostname == '10.1.2.2':

        connection_octus(title, destination_ip, source_ip, hostname, username, password)
    else:
        connection_mikrotik(title, destination_ip, source_ip, hostname, username, password)

def main():
    wich_model(argv[1],argv[2],argv[3],argv[4],argv[5],argv[6])

#__Main__
if  __name__ ==  '__main__':
    main()