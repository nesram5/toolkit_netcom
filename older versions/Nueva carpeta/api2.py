import sys
import paramiko
from time import sleep

def connection_mikrotik(title, destination_ip, source_ip, host, usern, passw):
    port = 22
    ssh_client = paramiko.SSHClient()
    ssh_client.set_missing_host_key_policy(paramiko.AutoAddPolicy())

    try:

        # Enable logging to a file
        ssh_client.connect(hostname=host, port=port, username=usern, password=passw)
        
        command = "ping {destination_ip} src-address= {source_ip}".format(destination_ip = destination_ip, source_ip = source_ip)
        
        stdin, stdout, stderr = ssh_client.exec_command(command, get_pty=True)

        for line in iter(stdout.readline, ""):
            print(line, end="" "{title}".format(title = title))
    
    except paramiko.AuthenticationException:
        print("Authentication failed, please verify your credentials.")
    except paramiko.SSHException as e:
        print("SSH connection failed:", str(e))
    except Exception as e:
        print("An error occurred:", str(e))
    finally:
        ssh_client.close()

def connection_octus(title,destination_ip,source_ip,hostname,username, password):
    
    port = 22
    ssh_client = paramiko.SSHClient()
    ssh_client.set_missing_host_key_policy(paramiko.AutoAddPolicy())

    try:
        ssh_client.connect(hostname=hostname, port=port, username=username, password=password)
        shell = ssh_client.invoke_shell()
        command = "ping {destination_ip} source {source_ip}".format(destination_ip = destination_ip, source_ip = source_ip)
        shell.send(command + '\n')

        while not shell.recv_ready():
            sleep(3)

        try:
            while True:
                response = shell.recv(1024).decode()
                print(response, "{title}".format(title = title))
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

def wich_model(title, destination_ip, source_ip, hostname, username, password):
    
    if hostname == '10.1.0.8':
        connection_octus(title, destination_ip, source_ip, hostname, username, password)
    else:
        connection_mikrotik(title, destination_ip, source_ip, hostname, username, password)

def main():
    wich_model(sys.argv[1],sys.argv[2],sys.argv[3],sys.argv[4],sys.argv[5],sys.argv[6])

#__Main__
if  __name__ ==  '__main__':
    main()