#!/usr/bin/env python3

from sympy import is_primitive_root
import json

def is_prime(n):
    if n <= 1:
        return False
    if n == 2:
        return True  # 2 is prime
    if n % 2 == 0:
        return False  # eliminate even numbers

    # Check only odd divisors from 3 to sqrt(n)
    for i in range(3, int(n**0.5) + 1, 2):
        if n % i == 0:
            return False
    return True

def find_large_prime(n, m):
    max_32_bit = 2**31 - 1
    m = int(m)
    for num in range(max_32_bit - (max_32_bit - 1) % (n * m), 1, -(n * m)):
        if is_prime(num):
            return num
    return None

def find_small_prime(n, m):
    max_32_bit = 2**31 - 1
    m = int(m)
    for num in range(1, max_32_bit, 1):
        if is_prime(num) and (num - 1)% m == 0 and (num - 1) % n == 0:
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

def update_test_values(n_g, n_i, m, n, filename = "class_table.json"):
    new_values = {
        "n_g": n_g,
        "n_i": n_i,
        "n": n,
        "m": m,
        "row_a": 0,
        "col_a": 0,
        "val_a": 0,
        "row_b": 0,
        "col_b": 0,
        "val_b": 0,
        "row_c": 0,
        "col_c": 0,
        "val_c": 0
    }

    # Load the existing JSON data
    with open(filename, 'r') as file:
        data = json.load(file)

    # Update the "test" key with new values
    data['test'] = new_values

    # Write the updated data back to the JSON file
    with open(filename, 'w') as file:
        json.dump(data, file, indent=4)

    print("Updated 'test' values in class_table.json")

def find_largest_prime_and_generator(largest_prime):
    if largest_prime:
        print(f"Prime number: {largest_prime}")

        # Set p to the largest prime found
        p = largest_prime
        g = 2

        # Loop to find the smallest primitive root
        while g < p:
            if is_primitive_root(g, p):
                print(f"Generator: {g}")
                return (p, g)  # Return the tuple (p, g)
            g += 1
    else:
        print("No such prime number found.")
        return None  # Return None if no prime is found
    
def update_rust_constants(p, g, file_path='src/math.rs'):
    # Read the existing content of the Rust file
    with open(file_path, 'r') as file:
        lines = file.readlines()

    # Update the lines for P and GENERATOR
    for i, line in enumerate(lines):
        if line.startswith('pub const P:'):
            lines[i] = f'pub const P: u64 = {p};\n'
        elif line.startswith('pub const GENERATOR:'):
            lines[i] = f'pub const GENERATOR: u64 = {g};\n'

    # Write the updated content back to the Rust file
    with open(file_path, 'w') as file:
        file.writelines(lines)

    print(f"Updated P to {p} and GENERATOR to {g} in {file_path}")


# Specify the path to your file
file_path = 'program.s'

# Count the instructions
n_i, n_g = count_instructions(file_path)
n_i = 32
t = n_i + 1
n = n_i + n_g + 1
m = ((n**2 - n ) / 2) - ((t**2 - t ) / 2)
# m = 2 * n_g

print(f"n_g: {n_g}\nn_i: {n_i}")
print(f"n: {n}\nm: {int(m)}")

# prime = find_large_prime(n, m)
prime = find_small_prime(n, m)
# prime = 4767673

(p, g) = find_largest_prime_and_generator(prime)


update_test_values(n_g, n_i, int(m), n)
update_rust_constants(p, g)

