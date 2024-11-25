.text
.globl store_register_instances
store_register_instances:
    # Store each register's value in its respective array
    la t0, x0_array
    sw x0, 0(t0)            # Store x0 in x0_array
    la t0, x1_array
    sw x1, 0(t0)            # Store x1 in x1_array
    la t0, x2_array
    sw x2, 0(t0)            # Store x2 in x2_array
    la t0, x3_array
    sw x3, 0(t0)            # Store x3 in x3_array
    la t0, x4_array
    sw x4, 0(t0)            # Store x4 in x4_array
    la t0, x5_array
    sw x5, 0(t0)            # Store x5 in x5_array
    la t0, x6_array
    sw x6, 0(t0)            # Store x6 in x6_array
    la t0, x7_array
    sw x7, 0(t0)            # Store x7 in x7_array
    la t0, x8_array
    sw x8, 0(t0)            # Store x8 in x8_array
    la t0, x9_array
    sw x9, 0(t0)            # Store x9 in x9_array
    la t0, x10_array
    sw x10, 0(t0)           # Store x10 in x10_array
    la t0, x11_array
    sw x11, 0(t0)           # Store x11 in x11_array
    la t0, x12_array
    sw x12, 0(t0)           # Store x12 in x12_array
    la t0, x13_array
    sw x13, 0(t0)           # Store x13 in x13_array
    la t0, x14_array
    sw x14, 0(t0)           # Store x14 in x14_array
    la t0, x15_array
    sw x15, 0(t0)           # Store x15 in x15_array
    la t0, x16_array
    sw x16, 0(t0)           # Store x16 in x16_array
    la t0, x17_array
    sw x17, 0(t0)           # Store x17 in x17_array
    la t0, x18_array
    sw x18, 0(t0)           # Store x18 in x18_array
    la t0, x19_array
    sw x19, 0(t0)           # Store x19 in x19_array
    la t0, x20_array
    sw x20, 0(t0)           # Store x20 in x20_array
    la t0, x21_array
    sw x21, 0(t0)           # Store x21 in x21_array
    la t0, x22_array
    sw x22, 0(t0)           # Store x22 in x22_array
    la t0, x23_array
    sw x23, 0(t0)           # Store x23 in x23_array
    la t0, x24_array
    sw x24, 0(t0)           # Store x24 in x24_array
    la t0, x25_array
    sw x25, 0(t0)           # Store x25 in x25_array
    la t0, x26_array
    sw x26, 0(t0)           # Store x26 in x26_array
    la t0, x27_array
    sw x27, 0(t0)           # Store x27 in x27_array
    la t0, x28_array
    sw x28, 0(t0)           # Store x28 in x28_array
    la t0, x29_array
    sw x29, 0(t0)           # Store x29 in x29_array
    la t0, x30_array
    sw x30, 0(t0)           # Store x30 in x30_array
    la t0, x31_array
    sw x31, 0(t0)           # Store x31 in x31_array

    ret                            # Return from function