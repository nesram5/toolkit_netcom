import os
file = [
    'C:\\SSH_Netcom\\scripts\\01-Ping-Fibex.py',
    'C:\\SSH_Netcom\\scripts\\02-Ping-Digitel-Carabobo.py',
    'C:\\SSH_Netcom\\scripts\\03-Ping-Digitel-Aragua.py',
    'C:\\SSH_Netcom\\scripts\\04-Ping-Inter-Val.py',
    'C:\\SSH_Netcom\\scripts\\05-Ping-Inter-Ara.py',
    'C:\\SSH_Netcom\\scripts\\06-TD-Digitel.py',
    'C:\\SSH_Netcom\\scripts\\07-TD-VLAN-625-PTO-FIBEX.py',
    'C:\\SSH_Netcom\\scripts\\08-TD-GUACARA.py',
    'C:\\SSH_Netcom\\scripts\\09-TD-MIRADOR.py',
    'C:\\SSH_Netcom\\scripts\\10-TD-Vista-Hermosa.py',
    'C:\\SSH_Netcom\\scripts\\11-TD-Caribe-Morita.py',
    'C:\\SSH_Netcom\\scripts\\12-TD-Caribe-VistaHermosa.py',
    'C:\\SSH_Netcom\\scripts\\13-TD-Interno-Dayco-Guacara.py',
    'C:\\SSH_Netcom\\scripts\\14-TD-Interno-Dayco-LosParques.py',
    'C:\\SSH_Netcom\\scripts\\15-TD-Interno-Dayco-Fundacion.py',
    'C:\\SSH_Netcom\\scripts\\16-TD-Interno-Dayco-Paseo.py',
    'C:\\SSH_Netcom\\scripts\\17-TD-Interno-Copey-Parral.py'
]

title = '5a8'

   #cmd = "start /B start cmd.exe @cmd /k python.exe {0}".format(file[j])
cmd = "start /B start wt.exe --title tabname1 python.exe {0} ; sp -V -c python.exe {1}; mf left ; sp -H -c python.exe {2}; mf right ; sp -H -c python.exe {3}".format(file[0],file[1],file[2],file[3])
os.system(cmd)

cmd = "start /B start wt.exe --title tabname2 python.exe {0} ; sp -V -c python.exe {1}; mf left ; sp -H -c python.exe {2}; mf right ; sp -H -c python.exe {3}".format(file[4],file[5],file[6],file[7])
os.system(cmd)

cmd = "start /B start wt.exe --title tabname3 python.exe {0} ; sp -V -c python.exe {1}; mf left ; sp -H -c python.exe {2}; mf right ; sp -H -c python.exe {3}".format(file[8],file[9],file[10],file[11])
os.system(cmd)

cmd = "start /B start wt.exe --title tabname4 python.exe {0} ; sp -V -c python.exe {1}; mf left ; sp -H -c python.exe {2}; mf right ; sp -H -c python.exe {3}; mf right ; sp -H -c python.exe {4}".format(file[12],file[13],file[14],file[15],file[16])
os.system(cmd)


#while i < 17:
#    #cmd = "start /B start cmd.exe @cmd /k python.exe {0}".format(file[j])
#    cmd = "start /b start wt.exe -w 2 nt --title {0} --c python.exe {1}".format(title[i],file[i])
#    os.system(cmd)
#    i+=1
