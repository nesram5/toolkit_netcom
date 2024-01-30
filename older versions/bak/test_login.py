import sys
from paramiko import SSHClient, AutoAddPolicy, SSHException, AuthenticationException
from time import sleep
import getpass
import bcrypt

def login():
    print("Login: \n")
    print("Ingrese su nombre de usuario:  ")
    usern = input()
    print("\n")
    password = getpass.getpass(prompt='Ingrese su contraseña: ')
    salt = bcrypt.gensalt()
    hashed_password = bcrypt.hashpw(password.encode(), salt)

    with open('password.txt', 'wb') as f:
        f.write(hashed_password)
    
    
    host = '10.0.0.10'
    port = 22
    ssh_client = SSHClient()
    ssh_client.set_missing_host_key_policy(AutoAddPolicy())

    try:

        # Enable logging to a file
        ssh_client.connect(hostname=host, port=port, username=usern, password=passw)

        print("Inicio de Sesion Correcto")
        
    except AuthenticationException:
        print("Authentication failed, please verify your credentials.")
    except SSHException as e:
        print("SSH connection failed:", str(e))
    except Exception as e:
        print("An error occurred:", str(e))
    finally:
        ssh_client.close()




def import_password():

    with open('password.txt', 'rb') as f:
        hashed_password = f.read()

    password = input('Ingrese su contraseña: ')

    if bcrypt.checkpw(password.encode(), hashed_password):
        print('Contraseña correcta')
    else:
        print('Contraseña incorrecta')

def line_to_string(line):
    data = line.replace('\n', ' ')
    return data

def test():
    ssh_client = SSHClient()
    ssh_client.set_missing_host_key_policy(AutoAddPolicy())
    hostname = '10.0.0.10'
    port = '22'
    username = 'nramirez'
    password='N3st0rR4m23*'

    ssh_client.connect(hostname=hostname, port=port, username=username, password=password)
        
    command = "ping 8.8.8.8 src-address= 45.182.141.0"
        
    stdin, stdout, stderr = ssh_client.exec_command(command, get_pty=True)
    i = 0
    data = []

    for line in iter(stdout.readline, ""):
        data.append(line_to_string(line))
        print(data, end="")
      
        
test()     