import paramiko
import sys
 # adding Folder_2/subfolder to the system path
sys.path.insert(1, 'C:\\SSH_Netcom\\radius\\')
from radius import user as username
from radius import passw as password

hostname = '10.0.0.10'
port = 22
ssh_client = paramiko.SSHClient()
ssh_client.set_missing_host_key_policy(paramiko.AutoAddPolicy())

try:

    # Enable logging to a file
    ssh_client.connect(hostname=hostname, port=port, username=username, password=password)
    
    stdin, stdout, stderr = ssh_client.exec_command('ping 8.8.8.8 src-address=206.1.88.1', get_pty=True)

    for line in iter(stdout.readline, ""):
       print(line, end="" "PING Proveedor InterCarabobo")
       
except paramiko.AuthenticationException:
    print("Authentication failed, please verify your credentials.")
except paramiko.SSHException as e:
    print("SSH connection failed:", str(e))
except Exception as e:
    print("An error occurred:", str(e))
finally:
    ssh_client.close()