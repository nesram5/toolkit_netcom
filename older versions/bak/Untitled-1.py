else:
        cmd = "start /b start wt.exe -w 1 split-pane -H -c python.exe {0}".format(file[j])
        os.system(cmd)
        j+=1
k = 4
while k < 8:
    if k%2==0:
        cmd = "start /b start wt.exe -w 1 split-pane -V -c python.exe {0}".format(file[k])
        os.system(cmd)
        k+=1
    else:
        cmd = "start /b start wt.exe -w 1 split-pane -H -c python.exe {0}".format(file[k])
        os.system(cmd)
        k+=1
i = 8
while i < 12:
    #cmd = "start /B start cmd.exe @cmd /k python.exe {0}".format(file[j])
    if i%2==0:
        cmd = "start /b start wt.exe -w 2 split-pane -V -c python.exe {0}".format(file[i])
        os.system(cmd)
        i+=1
    else:
        cmd = "start /b start wt.exe -w 2 split-pane -H -c python.exe {0}".format(file[i])
        os.system(cmd)
        i+=1
l = 12
while l < 17:
    if lk%2==0:
        cmd = "start /b start wt.exe -w 1 split-pane -V -c python.exe {0}".format(file[l])
        os.system(cmd)
        l+=1
    else:
        cmd = "start /b start wt.exe -w 1 split-pane -H -c python.exe {0}".format(file[l])
        os.system(cmd)
        l+=1
