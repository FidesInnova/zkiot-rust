import random

def generate_random_opcode(num_opcodes, file_path):
    # Define possible opcodes
    opcodes = ['mul', 'addi', 'div', 'sub']  # Only include 'mul' and 'addi' for subsequent lines
    
    with open(file_path, 'w') as file:
        # Generate the first opcode line with 'ld'
        address = f"{random.randint(0x40000000, 0x4FFFFFFF):08x}"
        immediate = random.randint(1, 2**31 - 1)  # Random immediate value
        file.write(f"{address}:       02f407b3                ld      R1, R1 ,{immediate}\n")
        
        # Generate the remaining random opcodes
        for _ in range(1, num_opcodes):
            address = f"{random.randint(0x40000000, 0x4FFFFFFF):08x}"
            opcode = random.choice(opcodes)
            immediate = random.randint(1, 2**31 - 1)  # Random immediate value
            file.write(f"{address}:       02f407b3                {opcode}    R1, R1 ,{immediate}\n")

# Specify the number of opcodes to generate and the output file path
num_opcodes = 20  # You can change this to generate more or fewer opcodes
file_path = 'sample.txt'  # Output file name
generate_random_opcode(num_opcodes, file_path)

print(f"Generated {num_opcodes} opcodes and saved to '{file_path}'.")
