import paramiko
import time
import sys
 # adding Folder_2/subfolder to the system path
sys.path.insert(1, 'C:\\SSH_Netcom\\radius\\')
from radius import user as username
from radius import passw as password

hostname = '10.1.0.8'
port = 22

ssh_client = paramiko.SSHClient()
ssh_client.set_missing_host_key_policy(paramiko.AutoAddPolicy())

try:

    ssh_client.connect(hostname=hostname, port=port, username=username, password=password)
    
    shell = ssh_client.invoke_shell()
    
    commands = [
        'ping 172.16.0.74 source 172.16.0.73'
    ]

    for command in commands:
        # Send command and wait for the shell prompt before sending the next command
        shell.send(command + '\n')

    while not shell.recv_ready():
        time.sleep(3)

    try:

        while True:
            response = shell.recv(1024).decode()
            print(response, "TD INTERNO DAY-PLI")
            if "condition" in response:
                break
    except KeyboardInterrupt:
        pass
    print("--- COMMAND EXECUTION FINISHED ---")

except paramiko.AuthenticationException:
    print("Authentication failed, please verify your credentials.")
except paramiko.SSHException as e:
    print("SSH connection failed:", str(e))
except Exception as e:
    print("An error occurred:", str(e))
finally:
    ssh_client.close()

    