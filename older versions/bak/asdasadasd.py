data_list = [
    '30 172.16.0.17                                56  64 13ms',
    '100 172.16.4.90                                56  64 198us',
    '114 1.1.1.1                                    56  58 37ms231us',
    '135 172.16.2.46                                56  64 471us',
    '134 172.16.0.73                                56  64 0ms'
]

ms_list = []
us_list = []

for data in data_list:
    elements = data.split()
    if len(elements) >= 5:
        value = elements[4]
        if 'ms' in value and 'us' in value:
            # Extract values for ms and us, and combine them into a float
            ms, us = value.split('ms')
            combined_value = float(ms) + float(us.rstrip('us')) / 1000.0
        elif 'ms' in value:
            # Save ms value to ms_list
            combined_value = float(value.rstrip('ms'))
            ms_list.append(combined_value)
        elif 'us' in value:
            # Save us value to us_list
            combined_value = float(value.rstrip('us'))
            us_list.append(combined_value)

        print("Combined Value:", combined_value)

# Now, ms_list and us_list contain the values accordingly.
print("ms_list:", ms_list)
print("us_list:", us_list)