from sys import argv
from paramiko import SSHClient, AutoAddPolicy, SSHException, AuthenticationException
from time import sleep

def connection_mikrotik(title, destination_ip, source_ip, host, usern, passw):
    port = 22
    ssh_client = SSHClient()
    ssh_client.set_missing_host_key_policy(AutoAddPolicy())

    try:

        # Enable logging to a file
        ssh_client.connect(hostname=host, port=port, username=usern, password=passw)
        
        command = "ping {destination_ip} src-address= {source_ip}".format(destination_ip = destination_ip, source_ip = source_ip)
        command = 'ip route print terse without-paging where gateway~"10.1.33"'
        
        stdin, stdout, stderr = ssh_client.exec_command(command, get_pty=True)

        for line in iter(stdout.readline, ""):
            print(line,)
    
    except AuthenticationException:
        print("Authentication failed, please verify your credentials.")
    except SSHException as e:
        print("SSH connection failed:", str(e))
    except Exception as e:
        print("An error occurred:", str(e))
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

        while not shell.recv_ready():
            sleep(3)

        try:
            while True:
                response = shell.recv(6625).decode()
                #response = shell.recv(1024).decode()
                print(response)
                print(print(colors.bg.lightgrey, "SKk", colors.fg.red, "Amartya" "{title}".format(title = title))
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