
    .section .text
    .global save_registers
    save_registers:
        la t0, registers     # Load the starting address of storage space
        # Storing register values
        sw zero, 0(t0)       # Store the value of register zero
        sw ra, 4(t0)         # Store the value of register ra
        sw sp, 8(t0)         # Store the value of register sp
        sw gp, 12(t0)        # Store the value of register gp
        sw tp, 16(t0)        # Store the value of register tp
        sw t0, 20(t0)        # Store the value of register t0
        sw t1, 24(t0)        # Store the value of register t1
        sw t2, 28(t0)        # Store the value of register t2
        sw s0, 32(t0)        # Store the value of register s0
        sw s1, 36(t0)        # Store the value of register s1
        sw s2, 40(t0)        # Store the value of register s2
        sw s3, 44(t0)        # Store the value of register s3
        sw s4, 48(t0)        # Store the value of register s4
        sw s5, 52(t0)        # Store the value of register s5
        sw s6, 56(t0)        # Store the value of register s6
        sw s7, 60(t0)        # Store the value of register s7
        sw s8, 64(t0)        # Store the value of register s8
        sw s9, 68(t0)        # Store the value of register s9
        sw s10, 72(t0)       # Store the value of register s10
        sw s11, 76(t0)       # Store the value of register s11
        sw a0, 80(t0)        # Store the value of register a0
        sw a1, 84(t0)        # Store the value of register a1
        sw a2, 88(t0)        # Store the value of register a2
        sw a3, 92(t0)        # Store the value of register a3
        sw a4, 96(t0)        # Store the value of register a4
        sw a5, 100(t0)       # Store the value of register a5
        sw a6, 104(t0)       # Store the value of register a6
        sw a7, 108(t0)       # Store the value of register a7
        ret                  # Return from the function
    
    call save_registers
    add    a1, a1, 5
    call save_registers
    mul    a2, a2, 2
    call save_registers
    sub    a2, a2, 10
    call save_registers
    mul    a1, a1, 7
    call save_registers
