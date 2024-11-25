#APP
jal store_register_instances
mul s2, s2, s3
la t0, x18_array
sw x18, 4(t0)
addi s1, s3, 11
la t0, x9_array
sw x9, 4(t0)
mul s2, s4, s4
la t0, x18_array
sw x18, 8(t0)
mul s3, s3, s4
la t0, x19_array
sw x19, 4(t0)
add s1, s2, s3
la t0, x9_array
sw x9, 8(t0)
add s3, s4, s4
la t0, x19_array
sw x19, 8(t0)
mul s2, s1, s1
la t0, x18_array
sw x18, 12(t0)
add s1, s2, s2
la t0, x9_array
sw x9, 12(t0)
la a0, z_array
li t0, 1
sw t0, 0(a0)
la a0, z_array
la a1, x0_array
lw t0, 0(a1)
sw t0, 4(a0)
la a0, z_array
la a1, x1_array
lw t0, 0(a1)
sw t0, 8(a0)
la a0, z_array
la a1, x2_array
lw t0, 0(a1)
sw t0, 12(a0)
la a0, z_array
la a1, x3_array
lw t0, 0(a1)
sw t0, 16(a0)
la a0, z_array
la a1, x4_array
lw t0, 0(a1)
sw t0, 20(a0)
la a0, z_array
la a1, x5_array
lw t0, 0(a1)
sw t0, 24(a0)
la a0, z_array
la a1, x6_array
lw t0, 0(a1)
sw t0, 28(a0)
la a0, z_array
la a1, x7_array
lw t0, 0(a1)
sw t0, 32(a0)
la a0, z_array
la a1, x8_array
lw t0, 0(a1)
sw t0, 36(a0)
la a0, z_array
la a1, x9_array
lw t0, 0(a1)
sw t0, 40(a0)
la a0, z_array
la a1, x10_array
lw t0, 0(a1)
sw t0, 44(a0)
la a0, z_array
la a1, x11_array
lw t0, 0(a1)
sw t0, 48(a0)
la a0, z_array
la a1, x12_array
lw t0, 0(a1)
sw t0, 52(a0)
la a0, z_array
la a1, x13_array
lw t0, 0(a1)
sw t0, 56(a0)
la a0, z_array
la a1, x14_array
lw t0, 0(a1)
sw t0, 60(a0)
la a0, z_array
la a1, x15_array
lw t0, 0(a1)
sw t0, 64(a0)
la a0, z_array
la a1, x16_array
lw t0, 0(a1)
sw t0, 68(a0)
la a0, z_array
la a1, x17_array
lw t0, 0(a1)
sw t0, 72(a0)
la a0, z_array
la a1, x18_array
lw t0, 0(a1)
sw t0, 76(a0)
la a0, z_array
la a1, x19_array
lw t0, 0(a1)
sw t0, 80(a0)
la a0, z_array
la a1, x20_array
lw t0, 0(a1)
sw t0, 84(a0)
la a0, z_array
la a1, x21_array
lw t0, 0(a1)
sw t0, 88(a0)
la a0, z_array
la a1, x22_array
lw t0, 0(a1)
sw t0, 92(a0)
la a0, z_array
la a1, x23_array
lw t0, 0(a1)
sw t0, 96(a0)
la a0, z_array
la a1, x24_array
lw t0, 0(a1)
sw t0, 100(a0)
la a0, z_array
la a1, x25_array
lw t0, 0(a1)
sw t0, 104(a0)
la a0, z_array
la a1, x26_array
lw t0, 0(a1)
sw t0, 108(a0)
la a0, z_array
la a1, x27_array
lw t0, 0(a1)
sw t0, 112(a0)
la a0, z_array
la a1, x28_array
lw t0, 0(a1)
sw t0, 116(a0)
la a0, z_array
la a1, x29_array
lw t0, 0(a1)
sw t0, 120(a0)
la a0, z_array
la a1, x30_array
lw t0, 0(a1)
sw t0, 124(a0)
la a0, z_array
la a1, x31_array
lw t0, 0(a1)
sw t0, 128(a0)
la a1, x18_array
lw t0, 4(a1)
sw t0, 132(a0)
la a1, x9_array
lw t0, 4(a1)
sw t0, 136(a0)
la a1, x18_array
lw t0, 8(a1)
sw t0, 140(a0)
la a1, x19_array
lw t0, 4(a1)
sw t0, 144(a0)
la a1, x9_array
lw t0, 8(a1)
sw t0, 148(a0)
la a1, x19_array
lw t0, 8(a1)
sw t0, 152(a0)
la a1, x18_array
lw t0, 12(a1)
sw t0, 156(a0)
la a1, x9_array
lw t0, 12(a1)
sw t0, 160(a0)
call proofGenerator
#NOAPP
.section .data
.global z_array
z_array:    .space 164
    .data
x0_array:    .space 4   # Array for x0
x1_array:    .space 4   # Array for x1
x2_array:    .space 4   # Array for x2
x3_array:    .space 4   # Array for x3
x4_array:    .space 4   # Array for x4
x5_array:    .space 4   # Array for x5
x6_array:    .space 4   # Array for x6
x7_array:    .space 4   # Array for x7
x8_array:    .space 4   # Array for x8
x9_array:    .space 16   # Array for x9
x10_array:    .space 4   # Array for x10
x11_array:    .space 4   # Array for x11
x12_array:    .space 4   # Array for x12
x13_array:    .space 4   # Array for x13
x14_array:    .space 4   # Array for x14
x15_array:    .space 4   # Array for x15
x16_array:    .space 4   # Array for x16
x17_array:    .space 4   # Array for x17
x18_array:    .space 16   # Array for x18
x19_array:    .space 12   # Array for x19
x20_array:    .space 4   # Array for x20
x21_array:    .space 4   # Array for x21
x22_array:    .space 4   # Array for x22
x23_array:    .space 4   # Array for x23
x24_array:    .space 4   # Array for x24
x25_array:    .space 4   # Array for x25
x26_array:    .space 4   # Array for x26
x27_array:    .space 4   # Array for x27
x28_array:    .space 4   # Array for x28
x29_array:    .space 4   # Array for x29
x30_array:    .space 4   # Array for x30
x31_array:    .space 4   # Array for x31
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
