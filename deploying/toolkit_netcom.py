from os import system, getcwd
current_dir = getcwd()
api = "{current_dir}\\api.exe".format(current_dir = current_dir)
radius = "{current_dir}\\radius.txt".format(current_dir = current_dir)
list_ip_file = "{current_dir}\\list_ip.txt".format(current_dir = current_dir)
list_ip = []
commands = []
username = ''
password = ''

def read_txt_to_string(file_path):
    with open(file_path, 'r') as file:
        data = file.read().replace('\n', ' ')
    return data

def read_txt_radius(file_path):
    with open(file_path, 'r') as file:
        data = file.read()
    return data
def command_list(list_ip, username, password):
    
    titletab = [
        '1-4',
        '5-8',
        '9-12',
        '13-16',
        '17-?'
    ]
    n = 0
    i = 0
    j = len(list_ip) -1
    try:
        while i < j:
            if i < j and i == 0: 
                cmd1 = "powershell.exe wt.exe --window new --title {title} {api} '{arg0}' '{arg1}' '{arg2}' '{arg3}' '{user}' '{passw}'".format(api = api, user = username, passw = password, arg0 = list_ip[i], arg1 = list_ip[i+1], arg2 = list_ip[i+2], arg3 = list_ip[i+3], title = titletab[n])
                i = i + 4
                commands.append(cmd1)
            elif  i < j :
                cmd1 = "powershell.exe wt.exe --window last new-tab --title {title} {api} '{arg0}' '{arg1}' '{arg2}' '{arg3}' '{user}' '{passw}'".format(api = api, user = username, passw = password, arg0 = list_ip[i], arg1 = list_ip[i+1], arg2 = list_ip[i+2], arg3 = list_ip[i+3], title = titletab[n])
                i = i + 4
                
                commands.append(cmd1)
            else:
                 break

            if i < j: 
                cmd2 = "powershell.exe wt.exe --window last sp --title {title} -V -c {api} '{arg4}' '{arg5}' '{arg6}' '{arg7}' '{user}' '{passw}'".format(api = api, user = username, passw = password, arg4 = list_ip[i], arg5 = list_ip[i+1], arg6 = list_ip[i+2], arg7 = list_ip[i+3], title = titletab[n])
                i = i + 4
                commands.append(cmd2)
            else:
                break
        
            cmd3 = "powershell.exe wt.exe --window last mf left"
            commands.append(cmd3)

            if i < j: 
                cmd4 = "powershell.exe wt.exe --window last sp --title {title} -H -c {api} '{arg8}' '{arg9}' '{arg10}' '{arg11}' '{user}' '{passw}' \n".format(api = api, user = username, passw = password, arg8 = list_ip[i], arg9 = list_ip[i+1], arg10 = list_ip[i+2], arg11 = list_ip[i+3], title = titletab[n])
                i = i + 4
                commands.append(cmd4)
            else:
                break

            cmd5 = "powershell.exe wt.exe --window last mf right" 
            commands.append(cmd5)

            if i < j: 
                cmd6 = "powershell.exe wt.exe --window last sp --title {title} -H -c {api} '{arg12}' '{arg13}' '{arg14}' '{arg15}' '{user}' '{passw}'" .format(api = api, user = username, passw = password, arg12 = list_ip[i], arg13 = list_ip[i+1], arg14 = list_ip[i+2], arg15 = list_ip[i+3], title = titletab[n])
                i = i + 4 
                n = n + 1 
                commands.append(cmd6)
            else:
                break
    except:
        print("error")
    return commands

def main():
    global radius, username, password, list_ip
    
    result= read_txt_radius(radius)
    credentials = result.split()
    username = credentials[0]
    password = credentials[1]

    result = read_txt_to_string(list_ip_file)
    list_ip = result.split()

    commands = command_list(list_ip, username, password)

    for command in commands:
        system(command)

def test():
    command_list()
    print(commands)

#__Main__
if  __name__ ==  '__main__':
    
    main()