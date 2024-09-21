def decode_base58(bc, length):
    base58_digits = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz'
    n = 0
    for char in bc:
        n = n * 58 + base58_digits.index(char)
    return n.to_bytes(length, 'big')

def find_correct_length_for_decoding(base58_string):
    for length in range(25, 50):  # Trying lengths from 25 to 50
        try:
            decoded_bytes = decode_base58(base58_string, length)
            return decoded_bytes.hex()
        except OverflowError:
            continue
    return None

# Base58 string to convert
base58_string = "6ZJDfSVjffvRYbtpFF33PSeYWNYJnbtMKiwWVt1YRjZW"

# Convert and get the hexadecimal string
hex_string = find_correct_length_for_decoding(base58_string)
print(hex_string)
