data_list = [
    '30 172.16.0.17                                56  64 13ms',
    '100 172.16.4.90                                56  64 198us',
    '114 1.1.1.1                                    56  58 37ms231us',
    '135 172.16.2.46                                56  64 471us',
    '134 172.16.0.73                                56  64 0ms'
]

# Initialize variables to store values
values_in_ms_and_us = []
values_not_in_ms_and_us = []

# Iterate through the list
for item in data_list:
    elements = item.split()
    fifth_element = elements[4]

    # Check the format of the fifth element
    if 'ms' in fifth_element and 'us' in fifth_element:
        ms_parts = fifth_element.split('ms')
        us_part = ms_parts[1].rstrip('us')
        value_combined = float(ms_parts[0]) + float(us_part) / 1000.0
        values_in_ms_and_us.append(value_combined)
    else:
        values_not_in_ms_and_us.append(float(fifth_element.rstrip('msus')))

# Print the results
print("Values in ms and us:", values_in_ms_and_us)
print("Values not in ms and us:", values_not_in_ms_and_us)
