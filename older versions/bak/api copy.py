from sys import argv
from paramiko import SSHClient, AutoAddPolicy, SSHException, AuthenticationException
from time import sleep

class colors:
    reset = '\033[0m'
    bold = '\033[01m'
    disable = '\033[02m'
    underline = '\033[04m'
    reverse = '\033[07m'
    strikethrough = '\033[09m'
    invisible = '\033[08m'
    
    class fg:
            black = '\033[30m'
            red = '\033[31m'
            green = '\033[32m'
            orange = '\033[33m'
            blue = '\033[34m'
            purple = '\033[35m'
            cyan = '\033[36m'
            lightgrey = '\033[37m'
            darkgrey = '\033[90m'
            lightred = '\033[91m'
            lightgreen = '\033[92m'
            yellow = '\033[93m'
            lightblue = '\033[94m'
            pink = '\033[95m'
            lightcyan = '\033[96m'
    
    class bg:
            black = '\033[40m'
            red = '\033[41m'
            green = '\033[42m'
            orange = '\033[43m'
            blue = '\033[44m'
            purple = '\033[45m'
            cyan = '\033[46m'
            lightgrey = '\033[47m'
    class fs:
        reset = '\x1b[0m',
        bold= '\x1b[1m',
        italic = '\x1b[3m',
        underline = '\x1b[4m',
        inverse = '\x1b[7m',

def connection_mikrotik(title, destination_ip, source_ip, host, usern, passw):
    port = 22
    ssh_client = SSHClient()
    ssh_client.set_missing_host_key_policy(AutoAddPolicy())

    try:

        # Enable logging to a file
        ssh_client.connect(hostname=host, port=port, username=usern, password=passw)
        
        command = "ping {destination_ip} src-address= {source_ip}".format(destination_ip = destination_ip, source_ip = source_ip)
        
        stdin, stdout, stderr = ssh_client.exec_command(command, get_pty=True)
        i = 0
        for line in iter(stdout.readline, ""):
            print(colors.bg.black, colors.fg.lightgrey, line, end="")
            i = i + 1
            while i == 8:
                print(colors.bg.lightgrey, colors.fg.blue,  "{title}" .format(title = title), colors.bg.black)          
                i = 0
    
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
                response = shell.recv(1024).decode()
                print(response)
                print(colors.bg.lightgrey, "{title}", colors.fg.red, "{title}" .format(title = title))
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

def login():
    usern = ''
    passw = ''
    host = '10.0.0.10'
    port = 22
    ssh_client = SSHClient()
    ssh_client.set_missing_host_key_policy(AutoAddPolicy())

    try:

        # Enable logging to a file
        ssh_client.connect(hostname=host, port=port, username=usern, password=passw)
        
        command = "ping {destination_ip} src-address= {source_ip}".format(destination_ip = destination_ip, source_ip = source_ip)
        
        stdin, stdout, stderr = ssh_client.exec_command(command, get_pty=True)
        i = 0
        for line in iter(stdout.readline, ""):
            print(colors.bg.black, colors.fg.lightgrey, line, end="")
            i = i + 1
            while i == 8:
                print(colors.bg.lightgrey, colors.fg.blue,  "{title}" .format(title = title), colors.bg.black)          
                i = 0
    
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