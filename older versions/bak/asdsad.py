input_string = 'sent=60 received=60 packet-loss=0% min-rtt=33ms437us avg-rtt=33ms460us max-rtt=33ms548us'

# Split the string based on spaces
elements = input_string.split()

# Initialize variables to store values
packet_loss_percentage = None
avg_rtt_ms = None
avg_rtt_us = None

# Iterate through elements to find and extract values
for element in elements:
    if 'packet-loss=' in element:
        packet_loss_percentage = float(element.split('=')[1].strip('%'))
    elif 'avg-rtt=' in element:
        avg_rtt_parts = element.split('=')[1].split('ms')
        avg_rtt_ms = float(avg_rtt_parts[0])
        avg_rtt_us = float(avg_rtt_parts[1].rstrip('us'))

# Combine avg-rtt values in one float
avg_rtt_combined = avg_rtt_ms + avg_rtt_us / 1000.0

# Print the results
print("Packet Loss Percentage:", packet_loss_percentage)
print("Combined Avg RTT:", avg_rtt_combined, "ms")

# Now, packet_loss_percentage and avg_rtt_combined variables hold the desired values.
