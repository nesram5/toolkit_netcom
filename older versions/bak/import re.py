import re

command = '\x1b[m    0 8.8.8.8                                    56 119 33ms469us \r\n'


string = '    \x1b[m\x1b[32msent\x1b[m\x1b[33m=\x1b[m60 \x1b[m\x1b[32mreceived\x1b[m\x1b[33m=\x1b[m60 \x1b[m\x1b[32mpacket-loss\x1b[m\x1b[33m=\x1b[m0% \x1b[m\x1b[32mmin-rtt\x1b[m\x1b[33m=\x1b[m33ms442us \x1b[m\x1b[32mavg-rtt\x1b[m\x1b[33m=\x1b[m33ms465us \r\n'

header = '\r\x1b[9999B\r\x1b[9999B\x1b[m\x1b[1m  SEQ HOST                                     SIZE TTL TIME       STATUS      \r\n'


string_size_in_bytes = len(string) * 2
print(string_size_in_bytes)
string_size_in_bytes = len(command) * 2
print(string_size_in_bytes)
string_size_in_bytes = len(header) * 2
print(string_size_in_bytes)
ms_part = re.search(r'\d+ms', command).group()
ms = re.sub(r'\D', '', ms_part)
us_part = re.search(r'\d+us', command).group()
us = re.sub(r'\D', '', us_part)
value_ttl = re.search(r'56\s+(\d+)', command).group(1)
ttl = re.sub(r'\D', '', value_ttl)
avg_rtt = re.search(r'avg-rtt\\x1b\[m\\x1b\[33m=\\x1b\[m(\d+ms)', string)
print(avg_rtt)
print(ttl)
result = int(ms  +  us)
print(f'The ms part of the string is: {result:,d} ms')
