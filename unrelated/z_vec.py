register_map = {
    "zero": 0,
    "ra": 0,   # Return address
    "sp": 0,   # Stack pointer
    "gp": 0,   # Global pointer
    "tp": 0,   # Thread pointer
    "t0": 0,   # Temporary register
    "t1": 0,   # Temporary register
    "t2": 0,   # Temporary register
    "s0": 0,   # Platform register
    "s1": 0,   # Platform register
    "a0": 0,  # Argument register
    "a1": 0,  # Argument register
    "a2": 0,  # Temporary register
    "a3": 0,  # Temporary register
    "a4": 0,  # Temporary register
    "a5": 0,  # Temporary register
    "a6": 0,  # Temporary register
    "a7": 0,  # Temporary register
    "s2": 2,  # Saved register
    "s3": 3,  # Saved register
    "s4": 4,  # Saved register
    "s5": 5,  # Saved register
    "s6": 0,  # Saved register
    "s7": 0,  # Saved register
    "s8": 0,  # Saved register
    "s9": 0,  # Saved register
    "s10": 0, # Saved register
    "s11": 0, # Saved register
    "t3": 0,  # Temporary register
    "t4": 0,  # Frame pointer
    "t5": 0,  # Temporary register
    "t6": 0   # Temporary register
}


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

x = []
y = []
t = []
for (inst, reg) in parsed_data:
    if inst == "addi":
        register_map[reg[0]] = register_map[reg[1]] + int(reg[2])
    if inst == "add":
        register_map[reg[0]] = register_map[reg[1]] + register_map[reg[2]]
    if inst == "mul":
        register_map[reg[0]] = register_map[reg[1]] * register_map[reg[2]]
    x.append((reg[0], register_map[reg[0]]))
    
print(register_map)

t = []
for i in range(len(x) - 1, 0, -1):
    if not x[i][0] in t:
        y.insert(0, x[i][1])
        t.append(x[i][0])
        x.pop(i)

x = [v[1] for v in x]

print(f"X: {x}")
print(f"Y: {y}")
print(f"Z: {x + y}")

