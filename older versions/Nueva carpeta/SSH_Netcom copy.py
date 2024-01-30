from os import system, getcwd
#from os import system
from radius import user as username
from radius import passw as password
from list_ip import *
current_dir = getcwd()
api = "{current_dir}\\api.exe".format(current_dir = current_dir)


commands = []
window_number = 1
i = 0

cmd1 = "powershell.exe wt.exe -w {window_number} {api} '{arg0}' '{arg1}' '{arg2}' '{arg3}' '{user}' '{passw}' \n".format(api = api, user = username, passw = password, arg0 = list_ip[0], arg1 = list_ip[1], arg2 = list_ip[2], arg3 = list_ip[3], window_number = window_number)

commands.append(cmd1)

cmd2 = "powershell.exe wt.exe -w {window_number} sp -V -c {api} '{arg4}' '{arg5}' '{arg6}' '{arg7}' '{user}' '{passw}' \n".format(api = api, user = username, passw = password, arg4 = list_ip[4], arg5 = list_ip[5], arg6 = list_ip[6], arg7 = list_ip[7], window_number = window_number)

commands.append(cmd2)

cmd3 = "powershell.exe wt.exe -w {window_number} powershell.exe wt.exe -w 1 mf left \n".format(window_number = window_number)

commands.append(cmd3)

cmd4 = " powershell.exe wt.exe -w {window_number} sp -H -c {api} '{arg8}' '{arg9}' '{arg10}' '{arg11}' '{user}' '{passw}' \n".format(api = api, user = username, passw = password, arg8 = list_ip[8], arg9 = list_ip[9], arg10 = list_ip[10], arg11 = list_ip[11],window_number = window_number)


commands.append(cmd4)

cmd5 = "powershell.exe wt.exe -w {window_number} mf right \n" .format(window_number = window_number)


commands.append(cmd5)

cmd6 = "powershell.exe wt.exe -w {window_number} sp -H -c {api} '{arg12}' '{arg13}' '{arg14}' '{arg15}' '{user}' '{passw}' \n" .format(api = api, user = username, passw = password, arg12 = list_ip[12], arg13 = list_ip[13], arg14 = list_ip[14], arg15 = list_ip[15], window_number = window_number)

commands.append(cmd6)



print(commands)

