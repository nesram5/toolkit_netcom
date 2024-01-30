test = '00ms000us'
string_size_in_bytes = len(test) * 2

print(string_size_in_bytes)


""" rtt = re.findall(r'\d+ms\d+us', line)[0]
ms = re.search(r'\d+ms', rtt)[0]#Grab Ms value
us = re.search(r'\d+us', rtt)[0] #Grab Us value

value_ttl = re.search(r'56\s+(\d+)', line).group() #Grab TTL value
ttl.append(re.sub(r'\D', '', value_ttl)) #save value of TLL for avg later
result = (float(ms  +  us) / 1000)
latency.append(result) #save value of latency for avg later"""