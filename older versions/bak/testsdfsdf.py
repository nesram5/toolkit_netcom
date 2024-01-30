line_list = [
    '775 172.17.2.130                               56  64 0ms',
    'sent=780 received=780 packet-loss=0% min-rtt=0ms avg-rtt=0ms max-rtt=6ms',
    'SEQ HOST                                     SIZE TTL TIME  STATUS',
    '780 172.17.2.130                               56  64 0us',
    '2                                                              could not...',
    'sent=20 received=0 packet-loss=100%',
    '775 172.17.2.130                               56  64 0ms',
    'sent=780 received=780 packet-loss=20% min-rtt=0ms avg-rtt=0ms max-rtt=6ms',
    'SEQ HOST                                     SIZE TTL TIME  STATUS',
    '780 172.17.2.130                               56  64 32ms777us',
    '2                                                              could not...',
    'sent=20 received=0 packet-loss=100%',
    '775 172.17.2.130                               56  64 0ms',
    'sent=780 received=780 packet-loss=0% min-rtt=0ms avg-rtt=0ms max-rtt=6ms',
    'SEQ HOST                                     SIZE TTL TIME  STATUS',
    '780 172.17.2.130                               56  64 0ms',
    '2                                                              could not...',
    'sent=20 received=0 packet-loss=100%'
]

for line in stdout:
    if 'sent=' in line and 'received=' in line and 'packet-loss=' in line:
        print("Ping Result Line:")
        print(line)
    elif 'SEQ HOST' in line:
        print("Header Line:")
        print(line)
    elif line.strip().endswith(('ms', 'us')):
        print("Individual Ping Result:")
        print(line)
    elif 'could not...' in line:
        print("Error Line:")
        print(line)
    elif 'packet-loss=100%' in line:
        print("Packet Loss Line:")
        print(line)
    else:
        print("Unknown Line:")
        print(line)