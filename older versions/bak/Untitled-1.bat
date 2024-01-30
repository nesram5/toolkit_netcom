#cmd = "start /B start wt.exe {api} '{arg0}' '{arg1}' '{arg2}' '{arg3}' '{user}' '{passw}'; sp -V -c {api} '{arg4}' '{arg5}' '{arg6}' '{arg7}' '{user}' '{passw}'; mf left ; sp -H -c {api} '{arg8}' '{arg9}' '{arg10}' '{arg11}' '{user}' '{passw}'; mf right ; sp -H -c {api} '{arg12}' '{arg13}' '{arg14}' '{arg15}' '{user}' '{passw}'".format(api = api, user = username, passw = password, arg0 = list_ip[0], arg1 = list_ip[1], arg2 = list_ip[2], arg3 = list_ip[3], arg4 = list_ip[4], arg5 = list_ip[5], arg6 = list_ip[6], arg7 = list_ip[7], arg8 = list_ip[8], arg9 = list_ip[9], arg10 = list_ip[10], arg11 = list_ip[11], arg12 = list_ip[12], arg13 = list_ip[13], arg14 = list_ip[14], arg15 = list_ip[15])

#cmd = "start /b start wt.exe /k {api} '{arg0}' '{arg1}' '{arg2}' '{arg3}' '{user}' '{passw}'".format(api = api, user = username, passw = password, arg0 = list_ip[0], arg1 = list_ip[1], arg2 = list_ip[2], arg3 = list_ip[3])



#system(cmd)

#cmd = "start /B start wt.exe --title tabname2 {api} {0} ; sp -V -c {api} {1}; mf left ; sp -H -c {api} {2}; mf right ; sp -H -c {api} {3}".format(file[4],file[5],file[6],file[7])
#system(cmd)

#cmd = "start /B start wt.exe --title tabname3 {api} {0} ; sp -V -c {api} {1}; mf left ; sp -H -c {api} {2}; mf right ; sp -H -c {api} {3}".format(file[8],file[9],file[10],file[11])
#system(cmd)

#cmd = "start /B start wt.exe --title tabname4 {api} {0} ; sp -V -c {api} {1}; mf left ; sp -H -c {api} {2}; mf right ; sp -H -c {api} {3}; mf right ; sp -H -c {api} {4}".format(file[12],file[13],file[14],file[15],file[16])
#system(cmd)


cmd1 = "powershell.exe wt.exe -w 1 {api} '{arg0}' '{arg1}' '{arg2}' '{arg3}' '{user}' '{passw}' \n".format(api = api, user = username, passw = password, arg0 = list_ip[0], arg1 = list_ip[1], arg2 = list_ip[2], arg3 = list_ip[3])

cmd2 = "powershell.exe wt.exe -w 1 sp -V -c {api} '{arg4}' '{arg5}' '{arg6}' '{arg7}' '{user}' '{passw}' \n".format(api = api, user = username, passw = password, arg4 = list_ip[4], arg5 = list_ip[5], arg6 = list_ip[6], arg7 = list_ip[7])
 
cmd3 = "powershell.exe wt.exe -w 1 powershell.exe wt.exe -w 1 mf left \n"

cmd4 = " powershell.exe wt.exe -w 1 sp -H -c {api} '{arg8}' '{arg9}' '{arg10}' '{arg11}' '{user}' '{passw}' \n".format(api = api, user = username, passw = password, arg8 = list_ip[8], arg9 = list_ip[9], arg10 = list_ip[10], arg11 = list_ip[11])

cmd5 = "powershell.exe wt.exe -w 1 mf right \n" 

cmd6 = "powershell.exe wt.exe -w 1 sp -H -c {api} '{arg12}' '{arg13}' '{arg14}' '{arg15}' '{user}' '{passw}' \n" .format(api = api, user = username, passw = password, arg12 = list_ip[12], arg13 = list_ip[13], arg14 = list_ip[14], arg15 = list_ip[15])

system(cmd1)
system(cmd2)
system(cmd3)
system(cmd4)
system(cmd5)
system(cmd6)
