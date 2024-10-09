#!/usr/bin/env python3

from sympy import is_primitive_root

def is_prime(n):
    if n <= 1:
        return False
    for i in range(2, int(n**0.5) + 1):
        if n % i == 0:
            return False
    return True

def find_largest_prime(n, m):
    max_32_bit = 2**31 - 1
    for num in range(max_32_bit, 1, -1):
        if is_prime(num) and (num - 1) % n == 0 and (num - 1) % m == 0:
            return num
    return None

def count_instructions(file_path):
    n_i = 0  # Counter for 'ld' instructions
    n_g = 0  # Counter for all other instructions

    with open(file_path, 'r') as file:
        for line in file:
            # Split the line into parts
            parts = line.split()
            if len(parts) >= 3:  # Ensure there are enough parts
                instruction = parts[2]  # The instruction is the third part
                if instruction == 'ld':
                    n_i += 1
                else:
                    n_g += 1

    return n_i, n_g


# Specify the path to your file
file_path = 'sample.txt'

# Count the instructions
n_i, n_g = count_instructions(file_path)

t = n_i + 1
n = n_i + n_g + 1
m = ((n**2 - n ) / 2) - ((t**2 - t ) / 2)

print(f"n_g: {n_g}\nn_i: {n_i}")
print(f"n: {n}\nm: {m}")

# Find the largest prime number
largest_prime = find_largest_prime(n, m)

if largest_prime:
    print(f"Largest prime number found: {largest_prime}")
    
    # Set p to the largest prime found
    p = largest_prime
    g = 2

    # Loop to find the smallest primitive root
    while g < p:
        if is_primitive_root(g, p):
            print(f"The smallest generator is: {g}")
            break
        g += 1
else:
    print("No such prime number found.")

