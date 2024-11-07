	.file	"main.cpp"
	.option nopic
	.attribute arch, "rv32i2p1_m2p0_a2p1_f2p2_d2p2_c2p0_zicsr2p0_zifencei2p0"
	.attribute unaligned_access, 0
	.attribute stack_align, 16
	.text
	.align	1
	.globl	main
	.type	main, @function
main:
.LFB0:
	.cfi_startproc
	addi	sp,sp,-32
	.cfi_def_cfa_offset 32
	sw	s0,28(sp)
	.cfi_offset 8, -4
	addi	s0,sp,32
	.cfi_def_cfa 8, 0
	li	a5,232
	sw	a5,-20(s0)
	lw	a5,-20(s0)
	addi	a5,a5,123
    #START_LINE
    addi s2, s2, 5
    mul  s3, s3, 2
    addi s3, s3, 10
    mul  s2, s2, 7
    #END_LINE
	sw	a5,-20(s0)
	lw	a5,-20(s0)
	addi	a5,a5,123
	sw	a5,-20(s0)
	lw	a5,-20(s0)
	addi	a5,a5,123
	sw	a5,-20(s0)
	li	a5,0
	mv	a0,a5
	lw	s0,28(sp)
	.cfi_restore 8
	.cfi_def_cfa 2, 32
	addi	sp,sp,32
	.cfi_def_cfa_offset 0
	jr	ra
	.cfi_endproc
.LFE0:
	.size	main, .-main
	.ident	"GCC: (13.2.0-11ubuntu1+12) 13.2.0"