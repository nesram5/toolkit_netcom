import os
import sys
import paramiko
import time
from radius import user as username
from radius import passw as password
from list_ip import *

def connection_mikrotik(title,destination_ip,source_ip,hostname,username,password):
    port = 22
    ssh_client = paramiko.SSHClient()
    ssh_client.set_missing_host_key_policy(paramiko.AutoAddPolicy())

    try:

        # Enable logging to a file
        ssh_client.connect(hostname=hostname, port=port, username=username, password=password)
        
        command = "ping {destination_ip} src-address={source_ip}".format(destination_ip,source_ip)
        
        stdin, stdout, stderr = ssh_client.exec_command(command, get_pty=True)

        for line in iter(stdout.readline, ""):
            print(line, end="" "{title}".format(title))
        
    except paramiko.AuthenticationException:
        print("Authentication failed, please verify your credentials.")
    except paramiko.SSHException as e:
        print("SSH connection failed:", str(e))
    except Exception as e:
        print("An error occurred:", str(e))
    finally:
        ssh_client.close()

def connection_octus(title,destination_ip,source_ip,hostname,username,password):
    
    port = 22
    ssh_client = paramiko.SSHClient()
    ssh_client.set_missing_host_key_policy(paramiko.AutoAddPolicy())

    try:
        ssh_client.connect(hostname=hostname, port=port, username=username, password=password)
        shell = ssh_client.invoke_shell()
        commands = [
            'ping {destination_ip} source {source_ip}'.format(destination_ip,source_ip)
        ]

        for command in commands:
            # Send command and wait for the shell prompt before sending the next command
            shell.send(command + '\n')

        while not shell.recv_ready():
            time.sleep(3)

        try:
            while True:
                response = shell.recv(1024).decode()
                print(response, "{title}".format(title))
                if "condition" in response:
                    break
        except KeyboardInterrupt:
            pass
        
    except paramiko.AuthenticationException:
        print("Authentication failed, please verify your credentials.")
    except paramiko.SSHException as e:
        print("SSH connection failed:", str(e))
    except Exception as e:
        print("An error occurred:", str(e))
    finally:
        ssh_client.close()

def wich_model(list_of_ip,username,password):
    if list_of_ip[3] == '10.1.0.8':
        connection_octus(list_of_ip[0],list_of_ip[1],list_of_ip[2],list_of_ip[3],username,password)
    else:
        connection_mikrotik(list_of_ip[0],list_of_ip[1],list_of_ip[2],list_of_ip[3],username,password)

#__Main__

wich_model(ping01,username,password)

Posible solucion. Cambiar Basededatos a una matriz, o diccionario