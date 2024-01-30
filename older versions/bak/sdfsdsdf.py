input_string = '0 8.8.8.8 56 118 33ms464us'
elements = input_string.split()

if len(elements) >= 5:
    fifth_element = elements[4]
    print("Value of the fifth element:", fifth_element)
else:
    print("The string does not have at least five elements.")
