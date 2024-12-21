# Copyright 2024 Fidesinnova, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import random
import sys


range_num = (1000, 9223372036854775073)

register_mapping = [
    ("zero", "x0"),
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
    opcodes = ['mul', 'add', 'addi']  # Only include 'mul' and 'addi' for subsequent lines
    
    with open(file_path, 'w') as file:
        # Generate the remaining random opcodes
        for _ in range(1, num_opcodes):
            opcode = random.choice(opcodes)
            reg_des = ""
            reg_lhs = ""
            reg_rhs = ""
            
            if opcode == 'add':
                reg_des = random.choice(register_mapping)[0]
                reg_lhs = random.choice(register_mapping)[0]
                reg_rhs = random.choice(register_mapping)[0]
                while reg_rhs == reg_lhs:
                    reg_rhs = random.choice(register_mapping)[0]

            if opcode == 'addi':
                reg_des = random.choice(register_mapping)[0]
                reg_lhs = random.choice(register_mapping)[0]
                reg_rhs = str(random.randint(range_num[0], range_num[1]))
                
            if opcode == 'mul':
                reg_des = random.choice(register_mapping)[0]
                reg_lhs = random.choice(register_mapping)[0]
                reg_rhs = random.choice(register_mapping)[0]
                
            file.write(f"{opcode:<8}{reg_des}, {reg_lhs}, {reg_rhs}\n")


def write_numbers_to_file(count, filename):
    with open(filename, 'w') as file:
        for number in range(1, count + 1):
            file.write(f"{number}\n")


# Specify the number of opcodes to generate and the output file path
num_opcodes = int(sys.argv[1])  # You can change this to generate more or fewer opcodes
file_path = 'program.s' 
generate_random_opcode(num_opcodes + 1, file_path)

print(f"Generated {num_opcodes} opcodes and saved to '{file_path}'.")
