import random
import sys


range_num = (1, 100)
register_mapping = [
    ("z", "z"),
    ("ra", "x1"),   # Return address
    ("sp", "x2"),   # Stack pointer
    ("gp", "x3"),   # Global pointer
    ("tp", "x4"),   # Thread pointer
    ("t0", "x5"),   # Temporary register
    ("t1", "x6"),   # Temporary register
    ("t2", "x7"),   # Temporary register
    ("s0", "x8"),   # Platform register
    ("s1", "x9"),   # Platform register
    ("a0", "x10"),  # Argument register
    ("a1", "x11"),  # Argument register
    ("a2", "x12"),  # Temporary register
    ("a3", "x13"),  # Temporary register
    ("a4", "x14"),  # Temporary register
    ("a5", "x15"),  # Temporary register
    ("a6", "x16"),  # Temporary register
    ("a7", "x17"),  # Temporary register
    ("s2", "x18"),  # Saved register
    ("s3", "x19"),  # Saved register
    ("s4", "x20"),  # Saved register
    ("s5", "x21"),  # Saved register
    ("s6", "x22"),  # Saved register
    ("s7", "x23"),  # Saved register
    ("s8", "x24"),  # Saved register
    ("s9", "x25"),  # Saved register
    ("s10", "x26"), # Saved register
    ("s11", "x27"), # Saved register
    ("t3", "x28"),  # Temporary register
    ("t4", "x29"),  # Frame pointer
    ("t5", "x30"),  # Temporary register
    ("t6", "x31"),  # Temporary register
]


def generate_random_opcode(num_opcodes, file_path):
    # Define possible opcodes
    opcodes = ['mul', 'add', 'sub', 'div']  # Only include 'mul' and 'addi' for subsequent lines
    
    with open(file_path, 'w') as file:
        for i in range(0, 32):
            address = f"{random.randint(0x40000000, 0x4FFFFFFF):08x}"
            immediate = random.randint(range_num[0], range_num[1])  # Random immediate value
            file.write(f"{address}:       02f407b3                ld     {register_mapping[i][0]}, {register_mapping[i][0]} ,{immediate}\n")
        
        # Generate the remaining random opcodes
        for _ in range(1, num_opcodes):
            address = f"{random.randint(0x40000000, 0x4FFFFFFF):08x}"
            opcode = random.choice(opcodes)
            immediate = random.randint(range_num[0], range_num[1])  # Random immediate value
            file.write(f"{address}:       02f407b3                {opcode}    a1, a1 ,{immediate}\n")

def write_numbers_to_file(count, filename):
    with open(filename, 'w') as file:
        for number in range(1, count + 1):
            file.write(f"{number}\n")


# Specify the number of opcodes to generate and the output file path
num_opcodes = int(sys.argv[1])  # You can change this to generate more or fewer opcodes
file_path = 'sample.txt' 
generate_random_opcode(num_opcodes, file_path)

write_numbers_to_file(num_opcodes, 'line_num.txt')
print(f"Generated {num_opcodes} opcodes and saved to '{file_path}'.")
