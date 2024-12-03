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

p = 4294967296
p = 18446744073709551615

p = 1678321

def write_vector_to_file(vector, filename):
    with open(filename, 'w') as file:
        # Join the vector elements as strings separated by commas and write to the file
        file.write(', '.join(map(str, vector)))



register_map = {
    "zero": 0,
    "ra": 1,   # Return address
    "sp": 2,   # Stack pointer
    "gp": 3,   # Global pointer
    "tp": 4,   # Thread pointer
    "t0": 5,   # Temporary register
    "t1": 6,   # Temporary register
    "t2": 7,   # Temporary register
    "s0": 8,   # Platform register
    "s1": 9,   # Platform register
    "a0": 10,  # Argument register
    "a1": 11,  # Argument register
    "a2": 12,  # Temporary register
    "a3": 13,  # Temporary register
    "a4": 14,  # Temporary register
    "a5": 15,  # Temporary register
    "a6": 16,  # Temporary register
    "a7": 17,  # Temporary register
    "s2": 18,  # Saved register
    "s3": 19,  # Saved register
    "s4": 20,  # Saved register
    "s5": 21,  # Saved register
    "s6": 22,  # Saved register
    "s7": 23,  # Saved register
    "s8": 24,  # Saved register
    "s9": 25,  # Saved register
    "s10": 26, # Saved register
    "s11": 27, # Saved register
    "t3": 28,  # Temporary register
    "t4": 29,  # Frame pointer
    "t5": 30,  # Temporary register
    "t6": 31   # Temporary register
}



# register_map = {
#     "zero": 7,
#     "ra": 11,   # Return address
#     "sp": 0,   # Stack pointer
#     "gp": 1,   # Global pointer
#     "tp": 0,   # Thread pointer
#     "t0": 1,   # Temporary register
#     "t1": 0,   # Temporary register
#     "t2": 0,   # Temporary register
#     "s0": 0,   # Platform register
#     "s1": 2,   # Platform register
#     "a0": 0,  # Argument register
#     "a1": 0,  # Argument register
#     "a2": 0,  # Temporary register
#     "a3": 0,  # Temporary register
#     "a4": 0,  # Temporary register
#     "a5": 0,  # Temporary register
#     "a6": 0,  # Temporary register
#     "a7": 0,  # Temporary register
#     "s2": 7,  # Saved register
#     "s3": 0,  # Saved register
#     "s4": 0,  # Saved register
#     "s5": 0,  # Saved register
#     "s6": 0,  # Saved register
#     "s7": 0,  # Saved register
#     "s8": 0,  # Saved register
#     "s9": 0,  # Saved register
#     "s10": 0, # Saved register
#     "s11": 0, # Saved register
#     "t3": 0,  # Temporary register
#     "t4": 0,  # Frame pointer
#     "t5": 0,  # Temporary register
#     "t6": 0   # Temporary register
# }


def parser(path: str):
    parsed_lines = [] 
    with open(path, "r") as program:
        for line in program:
            line = line.strip()  # Remove leading/trailing whitespace

            # Skip empty lines
            if not line or line[0] == "#":
                continue

            # Process lines regardless of section markers
            # Split the line into parts (instruction and operands)
            parts = line.split()
            instruction = parts[0]  # The instruction (e.g., 'mul', 'addi')
            operands = [operand.replace(",", "").strip() for operand in parts[1:]]  # The operands (e.g., 's2', 's3')

            # Store the instruction and its operands in a tuple
            parsed_lines.append((instruction, operands))
    return parsed_lines


parsed_data = parser("program.s")

x = [register_map[key] for key in register_map]

w = []
y = []
t = []
for (inst, reg) in parsed_data:
    if inst == "addi":
        register_map[reg[0]] = (register_map[reg[1]] + int(reg[2]))
    if inst == "add":
        register_map[reg[0]] = (register_map[reg[1]] + register_map[reg[2]]) 
    if inst == "mul":
        register_map[reg[0]] = (register_map[reg[1]] * register_map[reg[2]])
    w.append((reg[0], (register_map[reg[0]]) % p))
    
print(register_map)

t = []
for i in range(len(w) - 1, 0, -1):
    if not w[i][0] in t:
        y.insert(0, w[i][1])
        t.append(w[i][0])
        w.pop(i)

w = [v[1] for v in w]

print(f"X: {x}")
print(f"W: {w}")
print(f"Y: {y}")
z = [1] + x + w + y
print(f"Z: {z}")
write_vector_to_file(z, "proof_generation/z_vec.txt")

