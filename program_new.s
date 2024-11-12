	.text
	.attribute	4, 16
	.attribute	5, "rv64i2p1_m2p0_a2p1_f2p2_d2p2_c2p0_zicsr2p0_zmmul1p0"
	.file	"67lu8i1mr7ampavgpxd68b557"

	.data
	.globl	x18_array
x18_array:
	.zero	20


	.section	".text._ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h3ac5837e8ed948e2E","ax",@progbits
	.p2align	1
	.type	_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h3ac5837e8ed948e2E,@function
_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h3ac5837e8ed948e2E:
.Lfunc_begin0:
	.file	1 "/home/alirezza/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src" "result.rs"
	.loc	1 2006 0
	.cfi_startproc
	addi	sp, sp, -32
	.cfi_def_cfa_offset 32
	sd	a0, 0(sp)
.Ltmp0:
	.loc	1 2008 17 prologue_end
	ld	a0, 0(sp)
	sd	a0, 16(sp)
	sd	a0, 24(sp)
.Ltmp1:
	.loc	1 2008 23 is_stmt 0
	sd	a0, 8(sp)
.Ltmp2:
	.loc	1 2010 6 is_stmt 1
	ld	a0, 8(sp)
	.loc	1 2010 6 epilogue_begin is_stmt 0
	addi	sp, sp, 32
	ret
.Ltmp3:
.Lfunc_end0:
	.size	_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h3ac5837e8ed948e2E, .Lfunc_end0-_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h3ac5837e8ed948e2E
	.cfi_endproc

	.section	.text._ZN3std2rt10lang_start17h62fb49f3140921d1E,"ax",@progbits
	.hidden	_ZN3std2rt10lang_start17h62fb49f3140921d1E
	.globl	_ZN3std2rt10lang_start17h62fb49f3140921d1E
	.p2align	1
	.type	_ZN3std2rt10lang_start17h62fb49f3140921d1E,@function
_ZN3std2rt10lang_start17h62fb49f3140921d1E:
.Lfunc_begin1:
	.file	2 "/home/alirezza/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src" "rt.rs"
	.loc	2 188 0 is_stmt 1
	.cfi_startproc
	addi	sp, sp, -64
	.cfi_def_cfa_offset 64
	sd	ra, 56(sp)
	.cfi_offset ra, -8
	mv	a4, a3
	mv	a3, a2
	mv	a2, a1
	sd	a0, 16(sp)
	sd	a2, 24(sp)
	sd	a3, 32(sp)
	sb	a4, 47(sp)
.Ltmp4:
	.loc	2 195 10 prologue_end
	sd	a0, 8(sp)
.Lpcrel_hi0:
	.loc	2 194 17
	auipc	a0, %pcrel_hi(.L__unnamed_1)
	addi	a1, a0, %pcrel_lo(.Lpcrel_hi0)
	addi	a0, sp, 8
	call	_ZN3std2rt19lang_start_internal17h8f917e1637c20491E
	sd	a0, 0(sp)
	.loc	2 194 12 is_stmt 0
	ld	a0, 0(sp)
	sd	a0, 48(sp)
	ld	ra, 56(sp)
	.loc	2 201 2 epilogue_begin is_stmt 1
	addi	sp, sp, 64
	ret
.Ltmp5:
.Lfunc_end1:
	.size	_ZN3std2rt10lang_start17h62fb49f3140921d1E, .Lfunc_end1-_ZN3std2rt10lang_start17h62fb49f3140921d1E
	.cfi_endproc

	.section	".text._ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h5d866f0ed9f04cd9E","ax",@progbits
	.p2align	1
	.type	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h5d866f0ed9f04cd9E,@function
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h5d866f0ed9f04cd9E:
.Lfunc_begin2:
	.loc	2 195 0
	.cfi_startproc
	addi	sp, sp, -32
	.cfi_def_cfa_offset 32
	sd	ra, 24(sp)
	.cfi_offset ra, -8
	sd	a0, 8(sp)
.Ltmp6:
	.loc	2 195 70 prologue_end
	ld	a0, 0(a0)
	.loc	2 195 18 is_stmt 0
	call	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h678179e6fd03b5aaE
	call	_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$std..process..Termination$GT$6report17h7710fdf22934ad1eE
	sb	a0, 7(sp)
	addi	a0, sp, 7
.Ltmp7:
	.file	3 "/home/alirezza/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src" "process.rs"
	.loc	3 2053 9 is_stmt 1
	sd	a0, 16(sp)
.Ltmp8:
	.file	4 "/home/alirezza/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/pal/unix/process" "process_common.rs"
	.loc	4 636 9
	lbu	a0, 7(sp)
	ld	ra, 24(sp)
.Ltmp9:
	.loc	2 195 93 epilogue_begin
	addi	sp, sp, 32
	ret
.Ltmp10:
.Lfunc_end2:
	.size	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h5d866f0ed9f04cd9E, .Lfunc_end2-_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h5d866f0ed9f04cd9E
	.cfi_endproc

	.section	.text._ZN3std3sys9backtrace28__rust_begin_short_backtrace17h678179e6fd03b5aaE,"ax",@progbits
	.p2align	1
	.type	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h678179e6fd03b5aaE,@function
_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h678179e6fd03b5aaE:
.Lfunc_begin3:
	.file	5 "/home/alirezza/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys" "backtrace.rs"
	.loc	5 150 0
	.cfi_startproc
	addi	sp, sp, -32
	.cfi_def_cfa_offset 32
	sd	ra, 24(sp)
	.cfi_offset ra, -8
	sd	a0, 0(sp)
.Ltmp11:
	.loc	5 154 18 prologue_end
	call	_ZN4core3ops8function6FnOnce9call_once17h70ee1792a0f8136aE
	sd	a0, 16(sp)
.Ltmp12:
	.file	6 "/home/alirezza/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src" "hint.rs"
	.loc	6 389 5
	#APP
	#NO_APP
	ld	ra, 24(sp)
.Ltmp13:
	.loc	5 160 2 epilogue_begin
	addi	sp, sp, 32
	ret
.Ltmp14:
.Lfunc_end3:
	.size	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h678179e6fd03b5aaE, .Lfunc_end3-_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h678179e6fd03b5aaE
	.cfi_endproc

	.section	".text._ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he4f6803a51943c29E","ax",@progbits
	.p2align	1
	.type	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he4f6803a51943c29E,@function
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he4f6803a51943c29E:
.Lfunc_begin4:
	.file	7 "/home/alirezza/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops" "function.rs"
	.loc	7 250 0
	.cfi_startproc
	addi	sp, sp, -32
	.cfi_def_cfa_offset 32
	sd	ra, 24(sp)
	.cfi_offset ra, -8
	sd	a0, 16(sp)
.Ltmp15:
	.loc	7 250 5 prologue_end
	ld	a0, 0(a0)
	call	_ZN4core3ops8function6FnOnce9call_once17hdb553d2527f36714E
	ld	ra, 24(sp)
	.loc	7 250 5 epilogue_begin is_stmt 0
	addi	sp, sp, 32
	ret
.Ltmp16:
.Lfunc_end4:
	.size	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he4f6803a51943c29E, .Lfunc_end4-_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he4f6803a51943c29E
	.cfi_endproc

	.section	.text._ZN4core3ops8function6FnOnce9call_once17h70ee1792a0f8136aE,"ax",@progbits
	.p2align	1
	.type	_ZN4core3ops8function6FnOnce9call_once17h70ee1792a0f8136aE,@function
_ZN4core3ops8function6FnOnce9call_once17h70ee1792a0f8136aE:
.Lfunc_begin5:
	.loc	7 250 0 is_stmt 1
	.cfi_startproc
	addi	sp, sp, -32
	.cfi_def_cfa_offset 32
	sd	ra, 24(sp)
	.cfi_offset ra, -8
	sd	a0, 16(sp)
.Ltmp17:
	.loc	7 250 5 prologue_end
	jalr	a0
	ld	ra, 24(sp)
	.loc	7 250 5 epilogue_begin is_stmt 0
	addi	sp, sp, 32
	ret
.Ltmp18:
.Lfunc_end5:
	.size	_ZN4core3ops8function6FnOnce9call_once17h70ee1792a0f8136aE, .Lfunc_end5-_ZN4core3ops8function6FnOnce9call_once17h70ee1792a0f8136aE
	.cfi_endproc

	.section	.text._ZN4core3ops8function6FnOnce9call_once17hdb553d2527f36714E,"ax",@progbits
	.p2align	1
	.type	_ZN4core3ops8function6FnOnce9call_once17hdb553d2527f36714E,@function
_ZN4core3ops8function6FnOnce9call_once17hdb553d2527f36714E:
.Lfunc_begin6:
	.loc	7 250 0 is_stmt 1
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
	addi	sp, sp, -48
	.cfi_def_cfa_offset 48
	sd	ra, 40(sp)
	.cfi_offset ra, -8
	sd	a0, 8(sp)
.Ltmp19:
	addi	a0, sp, 8
.Ltmp22:
	.loc	7 250 5 prologue_end
	call	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h5d866f0ed9f04cd9E
.Ltmp20:
	sd	a0, 0(sp)
	j	.LBB6_3
.LBB6_1:
	ld	a0, 24(sp)
	call	_Unwind_Resume
.LBB6_2:
.Ltmp21:
	.loc	7 0 5 is_stmt 0
	sd	a0, 24(sp)
	sw	a1, 32(sp)
	j	.LBB6_1
.LBB6_3:
	ld	a0, 0(sp)
	ld	ra, 40(sp)
	.loc	7 250 5 epilogue_begin
	addi	sp, sp, 48
	ret
.Ltmp23:
.Lfunc_end6:
	.size	_ZN4core3ops8function6FnOnce9call_once17hdb553d2527f36714E, .Lfunc_end6-_ZN4core3ops8function6FnOnce9call_once17hdb553d2527f36714E
	.cfi_endproc
	.section	.gcc_except_table._ZN4core3ops8function6FnOnce9call_once17hdb553d2527f36714E,"a",@progbits
	.p2align	2, 0x0
GCC_except_table6:
.Lexception0:
	.byte	255
	.byte	255
	.byte	3
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.word	.Ltmp19-.Lfunc_begin6
	.word	.Ltmp20-.Ltmp19
	.word	.Ltmp21-.Lfunc_begin6
	.byte	0
	.word	.Ltmp20-.Lfunc_begin6
	.word	.Lfunc_end6-.Ltmp20
	.word	0
	.byte	0
.Lcst_end0:
	.p2align	2, 0x0

	.section	".text._ZN4core3ptr129drop_in_place$LT$std..rt..lang_start$LT$core..result..Result$LT$$LP$$RP$$C$anyhow..Error$GT$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h558dd7f1a2a5f091E","ax",@progbits
	.p2align	1
	.type	_ZN4core3ptr129drop_in_place$LT$std..rt..lang_start$LT$core..result..Result$LT$$LP$$RP$$C$anyhow..Error$GT$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h558dd7f1a2a5f091E,@function
_ZN4core3ptr129drop_in_place$LT$std..rt..lang_start$LT$core..result..Result$LT$$LP$$RP$$C$anyhow..Error$GT$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h558dd7f1a2a5f091E:
.Lfunc_begin7:
	.file	8 "/home/alirezza/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr" "mod.rs"
	.loc	8 521 0 is_stmt 1
	.cfi_startproc
	addi	sp, sp, -16
	.cfi_def_cfa_offset 16
	sd	a0, 8(sp)
.Ltmp24:
	.loc	8 521 1 prologue_end epilogue_begin
	addi	sp, sp, 16
	ret
.Ltmp25:
.Lfunc_end7:
	.size	_ZN4core3ptr129drop_in_place$LT$std..rt..lang_start$LT$core..result..Result$LT$$LP$$RP$$C$anyhow..Error$GT$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h558dd7f1a2a5f091E, .Lfunc_end7-_ZN4core3ptr129drop_in_place$LT$std..rt..lang_start$LT$core..result..Result$LT$$LP$$RP$$C$anyhow..Error$GT$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h558dd7f1a2a5f091E
	.cfi_endproc

	.section	".text._ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hb37c1fcd02863739E","ax",@progbits
	.p2align	1
	.type	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hb37c1fcd02863739E,@function
_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hb37c1fcd02863739E:
.Lfunc_begin8:
	.loc	3 2422 0
	.cfi_startproc
	addi	sp, sp, -16
	.cfi_def_cfa_offset 16
	li	a0, 0
.Ltmp26:
	.loc	3 2424 6 prologue_end epilogue_begin
	addi	sp, sp, 16
	ret
.Ltmp27:
.Lfunc_end8:
	.size	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hb37c1fcd02863739E, .Lfunc_end8-_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hb37c1fcd02863739E
	.cfi_endproc

	.section	".text._ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h8c8033d2b111120eE","ax",@progbits
	.p2align	1
	.type	_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h8c8033d2b111120eE,@function
_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h8c8033d2b111120eE:
.Lfunc_begin9:
	.loc	1 1994 0
	.cfi_startproc
	addi	sp, sp, -48
	.cfi_def_cfa_offset 48
	sd	a0, 8(sp)
.Ltmp28:
	.loc	1 1995 15 prologue_end
	ld	a0, 8(sp)
	.loc	1 1995 9 is_stmt 0
	bnez	a0, .LBB9_2
	j	.LBB9_1
.LBB9_1:
	.loc	1 0 9
	li	a0, 0
.Ltmp29:
	.loc	1 1996 22 is_stmt 1
	sd	a0, 16(sp)
.Ltmp30:
	.loc	1 1996 45 is_stmt 0
	j	.LBB9_3
.LBB9_2:
	.loc	1 1997 17 is_stmt 1
	ld	a0, 8(sp)
	sd	a0, 40(sp)
.Ltmp31:
	.loc	1 1997 42 is_stmt 0
	sd	a0, 24(sp)
	.loc	1 1997 23
	ld	a0, 24(sp)
	sd	a0, 16(sp)
.Ltmp32:
	.loc	1 1997 48
	j	.LBB9_3
.LBB9_3:
	.loc	1 1999 6 is_stmt 1
	ld	a0, 16(sp)
	.loc	1 1999 6 epilogue_begin is_stmt 0
	addi	sp, sp, 48
	ret
.Ltmp33:
.Lfunc_end9:
	.size	_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h8c8033d2b111120eE, .Lfunc_end9-_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h8c8033d2b111120eE
	.cfi_endproc

	.section	".text._ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$std..process..Termination$GT$6report17h7710fdf22934ad1eE","ax",@progbits
	.p2align	1
	.type	_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$std..process..Termination$GT$6report17h7710fdf22934ad1eE,@function
_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$std..process..Termination$GT$6report17h7710fdf22934ad1eE:
.Lfunc_begin10:
	.loc	3 2451 0 is_stmt 1
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception1
	addi	sp, sp, -192
	.cfi_def_cfa_offset 192
	sd	ra, 184(sp)
	.cfi_offset ra, -8
	sd	a0, 0(sp)
.Lpcrel_hi1:
.Ltmp40:
	.file	9 "/home/alirezza/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt" "mod.rs"
	.loc	9 349 9 prologue_end
	auipc	a0, %pcrel_hi(.L__unnamed_2)
	addi	a0, a0, %pcrel_lo(.Lpcrel_hi1)
	sd	a0, 128(sp)
.Ltmp41:
	.loc	3 2452 15
	ld	a0, 0(sp)
	.loc	3 2452 9 is_stmt 0
	bnez	a0, .LBB10_2
	j	.LBB10_1
.LBB10_1:
.Ltmp42:
	.loc	3 2453 24 is_stmt 1
	call	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hb37c1fcd02863739E
	sb	a0, 15(sp)
	j	.LBB10_3
.Ltmp43:
.LBB10_2:
	.loc	3 2454 17
	ld	a0, 0(sp)
	sd	a0, 16(sp)
	addi	a1, sp, 16
.Ltmp44:
	.loc	3 2455 45
	sd	a1, 136(sp)
.Lpcrel_hi2:
.Ltmp45:
	.file	10 "/home/alirezza/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt" "rt.rs"
	.loc	10 118 22
	auipc	a0, %got_pcrel_hi(_ZN6anyhow5error60_$LT$impl$u20$core..fmt..Debug$u20$for$u20$anyhow..Error$GT$3fmt17h21868fd0ae1b0132E)
	ld	a0, %pcrel_lo(.Lpcrel_hi2)(a0)
	sd	a0, 144(sp)
.Ltmp46:
	.file	11 "/home/alirezza/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr" "non_null.rs"
	.loc	11 234 18
	sd	a1, 152(sp)
.Ltmp47:
	.loc	10 103 17
	sd	a1, 104(sp)
	sd	a0, 112(sp)
	.loc	10 100 9
	ld	a0, 112(sp)
	sd	a0, 96(sp)
	ld	a0, 104(sp)
	sd	a0, 88(sp)
.Ltmp48:
	.loc	3 2455 45
	ld	a0, 96(sp)
	sd	a0, 80(sp)
	ld	a0, 88(sp)
	sd	a0, 72(sp)
	addi	a0, sp, 72
	sd	a0, 160(sp)
.Lpcrel_hi3:
.Ltmp49:
	.loc	9 353 9
	auipc	a1, %pcrel_hi(.L__unnamed_2)
	addi	a1, a1, %pcrel_lo(.Lpcrel_hi3)
	sd	a1, 24(sp)
	li	a1, 2
	sd	a1, 32(sp)
.Lpcrel_hi4:
	auipc	a1, %pcrel_hi(.L__unnamed_3)
	addi	a1, a1, %pcrel_lo(.Lpcrel_hi4)
	ld	a2, 0(a1)
	ld	a1, 8(a1)
	sd	a2, 56(sp)
	sd	a1, 64(sp)
	sd	a0, 40(sp)
	li	a0, 1
	sd	a0, 48(sp)
.Ltmp34:
	addi	a0, sp, 24
.Ltmp50:
	.loc	3 2455 17
	call	_ZN3std2io5stdio23attempt_print_to_stderr17h1e736052495fe582E
.Ltmp35:
	j	.LBB10_6
.Ltmp51:
.LBB10_3:
	.loc	3 2459 6
	lbu	a0, 15(sp)
	ld	ra, 184(sp)
	.loc	3 2459 6 epilogue_begin is_stmt 0
	addi	sp, sp, 192
	ret
.LBB10_4:
.Ltmp37:
	.loc	3 0 6
	addi	a0, sp, 16
	.loc	3 2457 13 is_stmt 1
	call	_ZN4core3ptr34drop_in_place$LT$anyhow..Error$GT$17hd10babe202f2c17dE
.Ltmp38:
	j	.LBB10_8
.LBB10_5:
.Ltmp36:
	.loc	3 0 13 is_stmt 0
	sd	a0, 168(sp)
	sw	a1, 176(sp)
	j	.LBB10_4
.LBB10_6:
	li	a0, 1
.Ltmp52:
	.loc	3 2456 17 is_stmt 1
	sb	a0, 15(sp)
	addi	a0, sp, 16
.Ltmp53:
	.loc	3 2457 13
	call	_ZN4core3ptr34drop_in_place$LT$anyhow..Error$GT$17hd10babe202f2c17dE
	j	.LBB10_3
.LBB10_7:
.Ltmp39:
	.loc	3 2451 5
	call	_ZN4core9panicking16panic_in_cleanup17h7e80a7119d98fe1cE
.LBB10_8:
	ld	a0, 168(sp)
	call	_Unwind_Resume
.Ltmp54:
.Lfunc_end10:
	.size	_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$std..process..Termination$GT$6report17h7710fdf22934ad1eE, .Lfunc_end10-_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$std..process..Termination$GT$6report17h7710fdf22934ad1eE
	.cfi_endproc
	.section	".gcc_except_table._ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$std..process..Termination$GT$6report17h7710fdf22934ad1eE","a",@progbits
	.p2align	2, 0x0
GCC_except_table10:
.Lexception1:
	.byte	255
	.byte	155
	.uleb128 .Lttbase0-.Lttbaseref0
.Lttbaseref0:
	.byte	3
	.uleb128 .Lcst_end1-.Lcst_begin1
.Lcst_begin1:
	.word	.Lfunc_begin10-.Lfunc_begin10
	.word	.Ltmp34-.Lfunc_begin10
	.word	0
	.byte	0
	.word	.Ltmp34-.Lfunc_begin10
	.word	.Ltmp35-.Ltmp34
	.word	.Ltmp36-.Lfunc_begin10
	.byte	0
	.word	.Ltmp37-.Lfunc_begin10
	.word	.Ltmp38-.Ltmp37
	.word	.Ltmp39-.Lfunc_begin10
	.byte	1
	.word	.Ltmp38-.Lfunc_begin10
	.word	.Lfunc_end10-.Ltmp38
	.word	0
	.byte	0
.Lcst_end1:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.section	.text._ZN16proof_generation4main17h92c5237958f168d4E,"ax",@progbits
	.p2align	1
	.type	_ZN16proof_generation4main17h92c5237958f168d4E,@function
_ZN16proof_generation4main17h92c5237958f168d4E:
.Lfunc_begin11:
	.file	12 "/home/alirezza/Documents/Projects/OtherProjects/Fides/zkpRust/zkIoT-Rust" "proof_generation/src/main.rs"
	.loc	12 25 0
	.cfi_startproc
	addi	sp, sp, -48
	.cfi_def_cfa_offset 48
	sd	ra, 40(sp)
	.cfi_offset ra, -8
.Ltmp55:
	.loc	12 28 9 prologue_end
	#APP
    jal store_register_instances
	addi	s2, s2, 12
    sw x18, x18_array(4)
	addi	s2, s2, 12
    sw x18, x18_array(8)
	addi	s2, s2, 12
    sw x18, x18_array(12)
	addi	s2, s2, 12
    sw x18, x18_array(16)
    jal proofGenerator
	#NO_APP
	.loc	12 37 5
	call	proofGenerator
	call	_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h8c8033d2b111120eE
	sd	a0, 16(sp)
	ld	a0, 16(sp)
	bnez	a0, .LBB11_2
	j	.LBB11_1
.LBB11_1:
	.loc	12 0 5 is_stmt 0
	li	a0, 0
	.loc	12 39 5 is_stmt 1
	sd	a0, 8(sp)
	.loc	12 40 2
	j	.LBB11_3
.LBB11_2:
	.loc	12 37 21
	ld	a0, 16(sp)
	sd	a0, 32(sp)
.Lpcrel_hi5:
.Ltmp56:
	.loc	12 37 5 is_stmt 0
	auipc	a1, %pcrel_hi(.L__unnamed_4)
	addi	a1, a1, %pcrel_lo(.Lpcrel_hi5)
	call	_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h3ac5837e8ed948e2E
	sd	a0, 8(sp)
	j	.LBB11_3
.Ltmp57:
.LBB11_3:
	.loc	12 40 2 is_stmt 1
	ld	a0, 8(sp)
	ld	ra, 40(sp)
	.loc	12 40 2 epilogue_begin is_stmt 0
	addi	sp, sp, 48
	ret
.Ltmp58:
.Lfunc_end11:
	.size	_ZN16proof_generation4main17h92c5237958f168d4E, .Lfunc_end11-_ZN16proof_generation4main17h92c5237958f168d4E
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	1
	.type	main,@function
main:
.Lfunc_begin12:
	.cfi_startproc
	addi	sp, sp, -16
	.cfi_def_cfa_offset 16
	sd	ra, 8(sp)
	.cfi_offset ra, -8
	mv	a2, a1
.Lpcrel_hi6:
	auipc	a1, %got_pcrel_hi(__rustc_debug_gdb_scripts_section__)
	ld	a1, %pcrel_lo(.Lpcrel_hi6)(a1)
	lbu	a1, 0(a1)
	sext.w	a1, a0
.Lpcrel_hi7:
	auipc	a0, %pcrel_hi(_ZN16proof_generation4main17h92c5237958f168d4E)
	addi	a0, a0, %pcrel_lo(.Lpcrel_hi7)
	li	a3, 0
	call	_ZN3std2rt10lang_start17h62fb49f3140921d1E
	ld	ra, 8(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end12:
	.size	main, .Lfunc_end12-main
	.cfi_endproc

	.type	.L__unnamed_1,@object
	.section	.data.rel.ro..L__unnamed_1,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_1:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he4f6803a51943c29E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h5d866f0ed9f04cd9E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h5d866f0ed9f04cd9E
	.size	.L__unnamed_1, 48

	.type	.L__unnamed_5,@object
	.section	.rodata..L__unnamed_5,"a",@progbits
.L__unnamed_5:
	.ascii	"Error: "
	.size	.L__unnamed_5, 7

	.type	.L__unnamed_6,@object
	.section	.rodata..L__unnamed_6,"a",@progbits
.L__unnamed_6:
	.byte	10
	.size	.L__unnamed_6, 1

	.type	.L__unnamed_2,@object
	.section	.data.rel.ro..L__unnamed_2,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_2:
	.quad	.L__unnamed_5
	.asciz	"\007\000\000\000\000\000\000"
	.quad	.L__unnamed_6
	.asciz	"\001\000\000\000\000\000\000"
	.size	.L__unnamed_2, 32

	.type	.L__unnamed_3,@object
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	3, 0x0
.L__unnamed_3:
	.zero	8
	.zero	8
	.size	.L__unnamed_3, 16

	.type	.L__unnamed_7,@object
	.section	.rodata..L__unnamed_7,"a",@progbits
.L__unnamed_7:
	.ascii	"proof_generation/src/main.rs"
	.size	.L__unnamed_7, 28

	.type	.L__unnamed_4,@object
	.section	.data.rel.ro..L__unnamed_4,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_4:
	.quad	.L__unnamed_7
	.asciz	"\034\000\000\000\000\000\000\000%\000\000\000\005\000\000"
	.size	.L__unnamed_4, 24

	.type	__rustc_debug_gdb_scripts_section__,@object
	.section	.debug_gdb_scripts,"aMS",@progbits,1,unique,1
	.weak	__rustc_debug_gdb_scripts_section__
__rustc_debug_gdb_scripts_section__:
	.asciz	"\001gdb_load_rust_pretty_printers.py"
	.size	__rustc_debug_gdb_scripts_section__, 34

	.section	.debug_abbrev,"",@progbits
	.byte	1
	.byte	17
	.byte	1
	.byte	37
	.byte	14
	.byte	19
	.byte	5
	.byte	3
	.byte	14
	.byte	16
	.byte	23
	.byte	27
	.byte	14
	.byte	17
	.byte	1
	.byte	85
	.byte	23
	.byte	0
	.byte	0
	.byte	2
	.byte	52
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.byte	2
	.byte	24
	.byte	0
	.byte	0
	.byte	3
	.byte	19
	.byte	1
	.byte	29
	.byte	19
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	4
	.byte	13
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.ascii	"\210\001"
	.byte	15
	.byte	56
	.byte	11
	.byte	0
	.byte	0
	.byte	5
	.byte	15
	.byte	0
	.byte	73
	.byte	19
	.byte	3
	.byte	14
	.byte	51
	.byte	6
	.byte	0
	.byte	0
	.byte	6
	.byte	36
	.byte	0
	.byte	3
	.byte	14
	.byte	62
	.byte	11
	.byte	11
	.byte	11
	.byte	0
	.byte	0
	.byte	7
	.byte	57
	.byte	1
	.byte	3
	.byte	14
	.byte	0
	.byte	0
	.byte	8
	.byte	19
	.byte	1
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	9
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	10
	.byte	52
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	14
	.ascii	"\210\001"
	.byte	15
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	11
	.byte	29
	.byte	1
	.byte	49
	.byte	19
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	88
	.byte	11
	.byte	89
	.byte	11
	.byte	87
	.byte	11
	.byte	0
	.byte	0
	.byte	12
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	49
	.byte	19
	.byte	0
	.byte	0
	.byte	13
	.byte	29
	.byte	1
	.byte	49
	.byte	19
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	88
	.byte	11
	.byte	89
	.byte	5
	.byte	87
	.byte	11
	.byte	0
	.byte	0
	.byte	14
	.byte	47
	.byte	0
	.byte	73
	.byte	19
	.byte	3
	.byte	14
	.byte	0
	.byte	0
	.byte	15
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	16
	.byte	19
	.byte	1
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.byte	50
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	17
	.byte	13
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.ascii	"\210\001"
	.byte	15
	.byte	56
	.byte	11
	.byte	50
	.byte	11
	.byte	0
	.byte	0
	.byte	18
	.byte	51
	.byte	1
	.byte	21
	.byte	19
	.byte	0
	.byte	0
	.byte	19
	.byte	13
	.byte	0
	.byte	73
	.byte	19
	.ascii	"\210\001"
	.byte	15
	.byte	56
	.byte	11
	.byte	52
	.byte	25
	.byte	0
	.byte	0
	.byte	20
	.byte	25
	.byte	1
	.byte	22
	.byte	11
	.byte	0
	.byte	0
	.byte	21
	.byte	19
	.byte	0
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.byte	50
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	22
	.byte	51
	.byte	1
	.byte	0
	.byte	0
	.byte	23
	.byte	25
	.byte	1
	.byte	0
	.byte	0
	.byte	24
	.byte	23
	.byte	1
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	25
	.byte	51
	.byte	0
	.byte	0
	.byte	0
	.byte	26
	.byte	46
	.byte	1
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	60
	.byte	25
	.byte	0
	.byte	0
	.byte	27
	.byte	5
	.byte	0
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	28
	.byte	11
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	0
	.byte	0
	.byte	29
	.byte	52
	.byte	0
	.byte	2
	.byte	24
	.byte	49
	.byte	19
	.byte	0
	.byte	0
	.byte	30
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	31
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	32
	.byte	52
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	33
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	34
	.byte	11
	.byte	1
	.byte	85
	.byte	23
	.byte	0
	.byte	0
	.byte	35
	.byte	52
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	14
	.ascii	"\210\001"
	.byte	15
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	36
	.byte	29
	.byte	1
	.byte	49
	.byte	19
	.byte	85
	.byte	23
	.byte	88
	.byte	11
	.byte	89
	.byte	5
	.byte	87
	.byte	11
	.byte	0
	.byte	0
	.byte	37
	.byte	21
	.byte	0
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	38
	.byte	46
	.byte	1
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	60
	.byte	25
	.byte	0
	.byte	0
	.byte	39
	.byte	46
	.byte	1
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	32
	.byte	11
	.byte	0
	.byte	0
	.byte	40
	.byte	5
	.byte	0
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	41
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	0
	.byte	0
	.byte	42
	.byte	25
	.byte	1
	.byte	22
	.byte	7
	.byte	0
	.byte	0
	.byte	43
	.byte	4
	.byte	1
	.byte	73
	.byte	19
	.byte	109
	.byte	25
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	44
	.byte	40
	.byte	0
	.byte	3
	.byte	14
	.byte	28
	.byte	15
	.byte	0
	.byte	0
	.byte	45
	.byte	46
	.byte	1
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	32
	.byte	11
	.byte	0
	.byte	0
	.byte	46
	.byte	52
	.byte	0
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	47
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	48
	.byte	21
	.byte	1
	.byte	0
	.byte	0
	.byte	49
	.byte	21
	.byte	1
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	50
	.byte	19
	.byte	0
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	51
	.byte	15
	.byte	0
	.byte	73
	.byte	19
	.byte	51
	.byte	6
	.byte	0
	.byte	0
	.byte	52
	.byte	1
	.byte	1
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	53
	.byte	33
	.byte	0
	.byte	73
	.byte	19
	.byte	34
	.byte	13
	.byte	55
	.byte	11
	.byte	0
	.byte	0
	.byte	54
	.byte	36
	.byte	0
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.byte	62
	.byte	11
	.byte	0
	.byte	0
	.byte	55
	.byte	46
	.byte	1
	.byte	71
	.byte	19
	.byte	32
	.byte	11
	.byte	0
	.byte	0
	.byte	56
	.byte	52
	.byte	0
	.byte	3
	.byte	14
	.ascii	"\210\001"
	.byte	15
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	57
	.byte	5
	.byte	0
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	58
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	106
	.byte	25
	.byte	0
	.byte	0
	.byte	0
	.section	.debug_info,"",@progbits
.Lcu_begin0:
	.word	.Ldebug_info_end0-.Ldebug_info_start0
.Ldebug_info_start0:
	.half	4
	.word	.debug_abbrev
	.byte	8
	.byte	1
	.word	.Linfo_string0
	.half	28
	.word	.Linfo_string1
	.word	.Lline_table_start0
	.word	.Linfo_string2
	.quad	0
	.word	.Ldebug_ranges2
	.byte	2
	.word	.Linfo_string3
	.word	61
	.byte	9
	.byte	3
	.quad	.L__unnamed_1
	.byte	3
	.word	181
	.word	.Linfo_string179
	.byte	48
	.byte	8
	.byte	4
	.word	.Linfo_string4
	.word	139
	.byte	8
	.byte	0
	.byte	4
	.word	.Linfo_string7
	.word	159
	.byte	8
	.byte	8
	.byte	4
	.word	.Linfo_string9
	.word	159
	.byte	8
	.byte	16
	.byte	4
	.word	.Linfo_string10
	.word	139
	.byte	8
	.byte	24
	.byte	4
	.word	.Linfo_string11
	.word	139
	.byte	8
	.byte	32
	.byte	4
	.word	.Linfo_string12
	.word	139
	.byte	8
	.byte	40
	.byte	0
	.byte	5
	.word	152
	.word	.Linfo_string6
	.word	0
	.byte	6
	.word	.Linfo_string5
	.byte	7
	.byte	0
	.byte	6
	.word	.Linfo_string8
	.byte	7
	.byte	8
	.byte	7
	.word	.Linfo_string13
	.byte	7
	.word	.Linfo_string14
	.byte	7
	.word	.Linfo_string15
	.byte	8
	.word	.Linfo_string178
	.byte	8
	.byte	8
	.byte	4
	.word	.Linfo_string16
	.word	1775
	.byte	8
	.byte	0
	.byte	0
	.byte	9
	.quad	.Lfunc_begin2
	.word	.Lfunc_end2-.Lfunc_begin2
	.byte	1
	.byte	82
	.word	.Linfo_string268
	.word	.Linfo_string269
	.byte	2
	.byte	195
	.word	6059
	.byte	10
	.byte	3
	.byte	145
	.byte	8
	.byte	6
	.word	.Linfo_string16
	.byte	1
	.byte	2
	.byte	189
	.word	1775
	.byte	11
	.word	6066
	.quad	.Ltmp7
	.word	.Ltmp9-.Ltmp7
	.byte	2
	.byte	195
	.byte	85
	.byte	12
	.byte	2
	.byte	145
	.byte	7
	.word	6072
	.byte	13
	.word	6098
	.quad	.Ltmp8
	.word	.Ltmp9-.Ltmp8
	.byte	3
	.half	2053
	.byte	16
	.byte	12
	.byte	2
	.byte	145
	.byte	16
	.word	6104
	.byte	0
	.byte	0
	.byte	14
	.word	1803
	.word	.Linfo_string21
	.byte	0
	.byte	0
	.byte	9
	.quad	.Lfunc_begin1
	.word	.Lfunc_end1-.Lfunc_begin1
	.byte	1
	.byte	82
	.word	.Linfo_string265
	.word	.Linfo_string266
	.byte	2
	.byte	188
	.word	6690
	.byte	15
	.byte	2
	.byte	145
	.byte	16
	.word	.Linfo_string16
	.byte	2
	.byte	189
	.word	1775
	.byte	15
	.byte	2
	.byte	145
	.byte	24
	.word	.Linfo_string299
	.byte	2
	.byte	190
	.word	6690
	.byte	15
	.byte	2
	.byte	145
	.byte	32
	.word	.Linfo_string300
	.byte	2
	.byte	191
	.word	6697
	.byte	15
	.byte	2
	.byte	145
	.byte	47
	.word	.Linfo_string302
	.byte	2
	.byte	192
	.word	6025
	.byte	14
	.word	1803
	.word	.Linfo_string21
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string72
	.byte	16
	.word	.Linfo_string169
	.byte	48
	.byte	1
	.byte	8
	.byte	17
	.word	.Linfo_string23
	.word	437
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string168
	.byte	48
	.byte	3
	.byte	8
	.byte	18
	.word	450
	.byte	19
	.word	4816
	.byte	8
	.byte	0

	.byte	20
	.byte	0
	.byte	4
	.word	.Linfo_string73
	.word	500
	.byte	8
	.byte	0
	.byte	0
	.byte	20
	.byte	1
	.byte	4
	.word	.Linfo_string74
	.word	508
	.byte	8
	.byte	0
	.byte	0
	.byte	20
	.byte	2
	.byte	4
	.word	.Linfo_string75
	.word	516
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	21
	.word	.Linfo_string73
	.byte	48
	.byte	3
	.byte	8
	.byte	21
	.word	.Linfo_string74
	.byte	48
	.byte	3
	.byte	8
	.byte	16
	.word	.Linfo_string75
	.byte	48
	.byte	3
	.byte	8
	.byte	17
	.word	.Linfo_string60
	.word	840
	.byte	8
	.byte	8
	.byte	3
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string142
	.byte	32
	.byte	3
	.byte	8
	.byte	17
	.word	.Linfo_string78
	.word	159
	.byte	8
	.byte	24
	.byte	3
	.byte	17
	.word	.Linfo_string79
	.word	5494
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string138
	.byte	56
	.byte	1
	.byte	8
	.byte	17
	.word	.Linfo_string82
	.word	604
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.word	.Linfo_string102
	.word	5545
	.byte	8
	.byte	32
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string101
	.byte	32
	.byte	3
	.byte	8
	.byte	22
	.byte	23
	.byte	4
	.word	.Linfo_string83
	.word	627
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string83
	.byte	32
	.byte	3
	.byte	8
	.byte	17
	.word	.Linfo_string60
	.word	978
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string134
	.byte	72
	.byte	3
	.byte	8
	.byte	17
	.word	.Linfo_string103
	.word	3075
	.byte	8
	.byte	32
	.byte	3
	.byte	17
	.word	.Linfo_string121
	.word	3179
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.word	.Linfo_string130
	.word	3276
	.byte	4
	.byte	56
	.byte	3
	.byte	17
	.word	.Linfo_string133
	.word	3276
	.byte	4
	.byte	64
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string128
	.byte	32
	.byte	3
	.byte	8
	.byte	18
	.word	719
	.byte	19
	.word	4816
	.byte	8
	.byte	0

	.byte	20
	.byte	0
	.byte	4
	.word	.Linfo_string122
	.word	755
	.byte	8
	.byte	0
	.byte	0
	.byte	20
	.byte	1
	.byte	4
	.word	.Linfo_string123
	.word	776
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string122
	.byte	32
	.byte	3
	.byte	8
	.byte	17
	.word	.Linfo_string60
	.word	5596
	.byte	8
	.byte	8
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string123
	.byte	32
	.byte	3
	.byte	8
	.byte	17
	.word	.Linfo_string60
	.word	5647
	.byte	8
	.byte	8
	.byte	3
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string143
	.byte	7
	.word	.Linfo_string144
	.byte	8
	.word	.Linfo_string146
	.byte	32
	.byte	8
	.byte	4
	.word	.Linfo_string145
	.word	538
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string76
	.byte	7
	.word	.Linfo_string77
	.byte	16
	.word	.Linfo_string167
	.byte	40
	.byte	1
	.byte	8
	.byte	14
	.word	538
	.word	.Linfo_string21
	.byte	14
	.word	808
	.word	.Linfo_string147
	.byte	17
	.word	.Linfo_string148
	.word	945
	.byte	4
	.byte	32
	.byte	3
	.byte	17
	.word	.Linfo_string159
	.word	3695
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	24
	.word	.Linfo_string165
	.byte	32
	.byte	8
	.byte	14
	.word	538
	.word	.Linfo_string21
	.byte	14
	.word	808
	.word	.Linfo_string147
	.byte	4
	.word	.Linfo_string155
	.word	3736
	.byte	8
	.byte	0
	.byte	4
	.word	.Linfo_string163
	.word	3766
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string148
	.byte	16
	.word	.Linfo_string158
	.byte	4
	.byte	1
	.byte	4
	.byte	17
	.word	.Linfo_string23
	.word	1159
	.byte	4
	.byte	0
	.byte	3
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string84
	.byte	7
	.word	.Linfo_string72
	.byte	16
	.word	.Linfo_string100
	.byte	32
	.byte	1
	.byte	8
	.byte	17
	.word	.Linfo_string23
	.word	1004
	.byte	8
	.byte	0
	.byte	2
	.byte	0
	.byte	7
	.word	.Linfo_string85
	.byte	16
	.word	.Linfo_string100
	.byte	32
	.byte	1
	.byte	8
	.byte	18
	.word	1017
	.byte	19
	.word	4816
	.byte	8
	.byte	0

	.byte	20
	.byte	0
	.byte	4
	.word	.Linfo_string86
	.word	1053
	.byte	8
	.byte	0
	.byte	0
	.byte	20
	.byte	1
	.byte	4
	.word	.Linfo_string90
	.word	1074
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string86
	.byte	32
	.byte	1
	.byte	8
	.byte	17
	.word	.Linfo_string60
	.word	5999
	.byte	8
	.byte	8
	.byte	1
	.byte	0
	.byte	16
	.word	.Linfo_string90
	.byte	32
	.byte	1
	.byte	8
	.byte	17
	.word	.Linfo_string91
	.word	6012
	.byte	8
	.byte	8
	.byte	1
	.byte	17
	.word	.Linfo_string98
	.word	6012
	.byte	8
	.byte	16
	.byte	1
	.byte	17
	.word	.Linfo_string99
	.word	6012
	.byte	8
	.byte	24
	.byte	1
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string87
	.byte	16
	.word	.Linfo_string88
	.byte	0
	.byte	1
	.byte	1
	.byte	25
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string149
	.byte	7
	.word	.Linfo_string76
	.byte	7
	.word	.Linfo_string148
	.byte	7
	.word	.Linfo_string150
	.byte	16
	.word	.Linfo_string158
	.byte	4
	.byte	1
	.byte	4
	.byte	17
	.word	.Linfo_string151
	.word	3637
	.byte	4
	.byte	0
	.byte	3
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string188
	.byte	7
	.word	.Linfo_string189
	.byte	7
	.word	.Linfo_string187
	.byte	7
	.word	.Linfo_string190
	.byte	16
	.word	.Linfo_string191
	.byte	1
	.byte	1
	.byte	1
	.byte	17
	.word	.Linfo_string60
	.word	6025
	.byte	1
	.byte	0
	.byte	3
	.byte	26
	.word	.Linfo_string196
	.word	.Linfo_string197
	.byte	4
	.half	635
	.word	6059

	.byte	27
	.word	6085
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string72
	.byte	9
	.quad	.Lfunc_begin3
	.word	.Lfunc_end3-.Lfunc_begin3
	.byte	1
	.byte	82
	.word	.Linfo_string270
	.word	.Linfo_string271
	.byte	5
	.byte	150
	.word	1803
	.byte	15
	.byte	2
	.byte	145
	.byte	0
	.word	.Linfo_string163
	.byte	5
	.byte	150
	.word	1775
	.byte	28
	.quad	.Ltmp12
	.word	.Ltmp13-.Ltmp12
	.byte	10
	.byte	2
	.byte	145
	.byte	16
	.word	.Linfo_string18
	.byte	1
	.byte	5
	.byte	154
	.word	1803
	.byte	11
	.word	4408
	.quad	.Ltmp12
	.word	.Ltmp13-.Ltmp12
	.byte	5
	.byte	157
	.byte	5
	.byte	29
	.byte	2
	.byte	145
	.byte	15
	.word	4430
	.byte	0
	.byte	0
	.byte	14
	.word	1775
	.word	.Linfo_string147
	.byte	14
	.word	1803
	.word	.Linfo_string21
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string187
	.byte	16
	.word	.Linfo_string191
	.byte	1
	.byte	1
	.byte	1
	.byte	17
	.word	.Linfo_string60
	.word	1203
	.byte	1
	.byte	0
	.byte	3
	.byte	26
	.word	.Linfo_string192
	.word	.Linfo_string193
	.byte	3
	.half	2052
	.word	6059

	.byte	27
	.word	1382
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string206
	.byte	30
	.quad	.Lfunc_begin8
	.word	.Lfunc_end8-.Lfunc_begin8
	.byte	1
	.byte	82
	.word	.Linfo_string281
	.word	.Linfo_string282
	.byte	3
	.half	2422
	.word	1382
	.byte	31
	.byte	2
	.byte	145
	.byte	14
	.byte	3
	.half	2422
	.word	152
	.byte	32
	.byte	2
	.byte	145
	.byte	15
	.word	.Linfo_string195
	.byte	3
	.half	2422
	.word	152
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string261
	.byte	30
	.quad	.Lfunc_begin10
	.word	.Lfunc_end10-.Lfunc_begin10
	.byte	1
	.byte	82
	.word	.Linfo_string294
	.word	.Linfo_string295
	.byte	3
	.half	2451
	.word	1382
	.byte	33
	.byte	2
	.byte	145
	.byte	0
	.word	.Linfo_string195
	.byte	3
	.half	2451
	.word	1803
	.byte	34
	.word	.Ldebug_ranges0
	.byte	35
	.byte	2
	.byte	145
	.byte	16
	.word	.Linfo_string305
	.byte	1
	.byte	3
	.half	2454
	.word	4828
	.byte	36
	.word	6446
	.word	.Ldebug_ranges1
	.byte	3
	.half	2455
	.byte	45
	.byte	12
	.byte	3
	.byte	145
	.ascii	"\240\001"
	.word	6452
	.byte	29
	.byte	3
	.byte	145
	.ascii	"\200\001"
	.word	6464
	.byte	0
	.byte	13
	.word	6491
	.quad	.Ltmp45
	.word	.Ltmp48-.Ltmp45
	.byte	3
	.half	2455
	.byte	69
	.byte	12
	.byte	3
	.byte	145
	.ascii	"\210\001"
	.word	6506
	.byte	11
	.word	6587
	.quad	.Ltmp46
	.word	.Ltmp48-.Ltmp46
	.byte	10
	.byte	118
	.byte	9
	.byte	12
	.byte	3
	.byte	145
	.ascii	"\210\001"
	.word	6602
	.byte	12
	.byte	3
	.byte	145
	.ascii	"\220\001"
	.word	6613
	.byte	11
	.word	2607
	.quad	.Ltmp46
	.word	.Ltmp47-.Ltmp46
	.byte	10
	.byte	104
	.byte	24
	.byte	12
	.byte	3
	.byte	145
	.ascii	"\210\001"
	.word	2633
	.byte	13
	.word	6531
	.quad	.Ltmp46
	.word	.Ltmp47-.Ltmp46
	.byte	11
	.half	1623
	.byte	9
	.byte	12
	.byte	3
	.byte	145
	.ascii	"\210\001"
	.word	6546
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	28
	.quad	.Ltmp42
	.word	.Ltmp43-.Ltmp42
	.byte	32
	.byte	3
	.byte	145
	.asciz	"\377"
	.word	.Linfo_string304
	.byte	3
	.half	2453
	.word	152
	.byte	0
	.byte	14
	.word	152
	.word	.Linfo_string21
	.byte	14
	.word	4828
	.word	.Linfo_string26
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.word	1788
	.word	.Linfo_string177
	.word	0
	.byte	37
	.word	1803
	.byte	7
	.word	.Linfo_string17
	.byte	7
	.word	.Linfo_string18
	.byte	16
	.word	.Linfo_string176
	.byte	8
	.byte	1
	.byte	8
	.byte	18
	.word	1816
	.byte	19
	.word	4816
	.byte	8
	.byte	0

	.byte	20
	.byte	0
	.byte	4
	.word	.Linfo_string20
	.word	1851
	.byte	8
	.byte	0
	.byte	0
	.byte	23
	.byte	4
	.word	.Linfo_string175
	.word	1890
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string20
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.word	152
	.word	.Linfo_string21
	.byte	14
	.word	4828
	.word	.Linfo_string26
	.byte	17
	.word	.Linfo_string60
	.word	152
	.byte	1
	.byte	0
	.byte	1
	.byte	0
	.byte	16
	.word	.Linfo_string175
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.word	152
	.word	.Linfo_string21
	.byte	14
	.word	4828
	.word	.Linfo_string26
	.byte	17
	.word	.Linfo_string60
	.word	4828
	.byte	8
	.byte	0
	.byte	1
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string186
	.byte	30
	.quad	.Lfunc_begin0
	.word	.Lfunc_end0-.Lfunc_begin0
	.byte	1
	.byte	82
	.word	.Linfo_string263
	.word	.Linfo_string264
	.byte	1
	.half	2006
	.word	1803
	.byte	33
	.byte	2
	.byte	145
	.byte	0
	.word	.Linfo_string297
	.byte	1
	.half	2006
	.word	2296
	.byte	28
	.quad	.Ltmp1
	.word	.Ltmp2-.Ltmp1
	.byte	35
	.byte	2
	.byte	145
	.byte	16
	.word	.Linfo_string298
	.byte	1
	.byte	1
	.half	2008
	.word	4828
	.byte	0
	.byte	14
	.word	152
	.word	.Linfo_string21
	.byte	14
	.word	4828
	.word	.Linfo_string26
	.byte	14
	.word	4828
	.word	.Linfo_string147
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string207
	.byte	30
	.quad	.Lfunc_begin9
	.word	.Lfunc_end9-.Lfunc_begin9
	.byte	1
	.byte	82
	.word	.Linfo_string283
	.word	.Linfo_string284
	.byte	1
	.half	1994
	.word	4670
	.byte	33
	.byte	2
	.byte	145
	.byte	8
	.word	.Linfo_string195
	.byte	1
	.half	1994
	.word	1803
	.byte	28
	.quad	.Ltmp29
	.word	.Ltmp30-.Ltmp29
	.byte	32
	.byte	2
	.byte	145
	.byte	39
	.word	.Linfo_string153
	.byte	1
	.half	1996
	.word	152
	.byte	0
	.byte	28
	.quad	.Ltmp31
	.word	.Ltmp32-.Ltmp31
	.byte	35
	.byte	2
	.byte	145
	.byte	40
	.word	.Linfo_string298
	.byte	1
	.byte	1
	.half	1997
	.word	4828
	.byte	0
	.byte	14
	.word	152
	.word	.Linfo_string21
	.byte	14
	.word	4828
	.word	.Linfo_string26
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string229
	.byte	1
	.byte	1
	.byte	1
	.byte	18
	.word	2181
	.byte	19
	.word	6025
	.byte	1
	.byte	0

	.byte	20
	.byte	0
	.byte	4
	.word	.Linfo_string20
	.word	2217
	.byte	1
	.byte	0
	.byte	0
	.byte	20
	.byte	1
	.byte	4
	.word	.Linfo_string175
	.word	2256
	.byte	1
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string20
	.byte	1
	.byte	1
	.byte	1
	.byte	14
	.word	152
	.word	.Linfo_string21
	.byte	14
	.word	4313
	.word	.Linfo_string26
	.byte	17
	.word	.Linfo_string60
	.word	152
	.byte	1
	.byte	1
	.byte	1
	.byte	0
	.byte	16
	.word	.Linfo_string175
	.byte	1
	.byte	1
	.byte	1
	.byte	14
	.word	152
	.word	.Linfo_string21
	.byte	14
	.word	4313
	.word	.Linfo_string26
	.byte	17
	.word	.Linfo_string60
	.word	4313
	.byte	1
	.byte	1
	.byte	1
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string289
	.byte	8
	.byte	1
	.byte	8
	.byte	22
	.byte	23
	.byte	4
	.word	.Linfo_string20
	.word	2332
	.byte	8
	.byte	0
	.byte	0
	.byte	23
	.byte	4
	.word	.Linfo_string175
	.word	2371
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string20
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.word	4804
	.word	.Linfo_string21
	.byte	14
	.word	4828
	.word	.Linfo_string26
	.byte	17
	.word	.Linfo_string60
	.word	4804
	.byte	1
	.byte	0
	.byte	1
	.byte	0
	.byte	16
	.word	.Linfo_string175
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.word	4804
	.word	.Linfo_string21
	.byte	14
	.word	4828
	.word	.Linfo_string26
	.byte	17
	.word	.Linfo_string60
	.word	4828
	.byte	8
	.byte	0
	.byte	1
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string24
	.byte	7
	.word	.Linfo_string32
	.byte	16
	.word	.Linfo_string37
	.byte	16
	.byte	1
	.byte	8
	.byte	14
	.word	5198
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string33
	.word	5205
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string44
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.word	5016
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string33
	.word	5307
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string55
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.word	152
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string33
	.word	139
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string110
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.word	6025
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string33
	.word	6032
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string251
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.word	4828
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string33
	.word	6518
	.byte	8
	.byte	0
	.byte	3
	.byte	38
	.word	.Linfo_string252
	.word	.Linfo_string253
	.byte	11
	.byte	232
	.word	2542

	.byte	14
	.word	4828
	.word	.Linfo_string21
	.byte	27
	.word	6478
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string255
	.byte	39
	.word	.Linfo_string256
	.word	.Linfo_string257
	.byte	11
	.half	1622
	.word	2542
	.byte	1
	.byte	14
	.word	4828
	.word	.Linfo_string21
	.byte	40
	.word	.Linfo_string254
	.byte	11
	.half	1622
	.word	6478
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string108
	.byte	16
	.word	.Linfo_string113
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.word	6025
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string33
	.word	2512
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.word	.Linfo_string111
	.word	2803
	.byte	1
	.byte	8
	.byte	3
	.byte	0
	.byte	0
	.byte	41
	.quad	.Lfunc_begin7
	.word	.Lfunc_end7-.Lfunc_begin7
	.byte	1
	.byte	82
	.word	.Linfo_string279
	.word	.Linfo_string280
	.byte	8
	.half	521
	.byte	31
	.byte	2
	.byte	145
	.byte	8
	.byte	8
	.half	521
	.word	6710
	.byte	14
	.word	181
	.word	.Linfo_string21
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string39
	.byte	16
	.word	.Linfo_string41
	.byte	0
	.byte	1
	.byte	1
	.byte	14
	.word	5277
	.word	.Linfo_string21
	.byte	0
	.byte	16
	.word	.Linfo_string46
	.byte	0
	.byte	1
	.byte	1
	.byte	14
	.word	5320
	.word	.Linfo_string21
	.byte	0
	.byte	16
	.word	.Linfo_string57
	.byte	0
	.byte	1
	.byte	1
	.byte	14
	.word	5416
	.word	.Linfo_string21
	.byte	0
	.byte	16
	.word	.Linfo_string112
	.byte	0
	.byte	1
	.byte	1
	.byte	14
	.word	6025
	.word	.Linfo_string21
	.byte	0
	.byte	16
	.word	.Linfo_string125
	.byte	0
	.byte	1
	.byte	1
	.byte	14
	.word	6045
	.word	.Linfo_string21
	.byte	0
	.byte	16
	.word	.Linfo_string135
	.byte	0
	.byte	1
	.byte	1
	.byte	14
	.word	649
	.word	.Linfo_string21
	.byte	0
	.byte	16
	.word	.Linfo_string139
	.byte	0
	.byte	1
	.byte	1
	.byte	14
	.word	571
	.word	.Linfo_string21
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string53
	.byte	16
	.word	.Linfo_string61
	.byte	8
	.byte	1
	.byte	8
	.byte	18
	.word	2894
	.byte	19
	.word	4816
	.byte	8
	.byte	0

	.byte	20
	.byte	0
	.byte	4
	.word	.Linfo_string54
	.word	2929
	.byte	8
	.byte	0
	.byte	0
	.byte	23
	.byte	4
	.word	.Linfo_string59
	.word	2947
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string54
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.word	4968
	.word	.Linfo_string21
	.byte	0
	.byte	16
	.word	.Linfo_string59
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.word	4968
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string60
	.word	4968
	.byte	8
	.byte	0
	.byte	1
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string170
	.byte	48
	.byte	1
	.byte	8
	.byte	18
	.word	2991
	.byte	19
	.word	4816
	.byte	8
	.byte	0

	.byte	20
	.byte	3
	.byte	4
	.word	.Linfo_string54
	.word	3026
	.byte	8
	.byte	0
	.byte	0
	.byte	23
	.byte	4
	.word	.Linfo_string59
	.word	3044
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string54
	.byte	48
	.byte	1
	.byte	8
	.byte	14
	.word	416
	.word	.Linfo_string21
	.byte	0
	.byte	16
	.word	.Linfo_string59
	.byte	48
	.byte	1
	.byte	8
	.byte	14
	.word	416
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string60
	.word	416
	.byte	8
	.byte	0
	.byte	1
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string120
	.byte	24
	.byte	1
	.byte	8
	.byte	18
	.word	3088
	.byte	19
	.word	4816
	.byte	8
	.byte	0

	.byte	42
	.quad	-9223372036854775808
	.byte	4
	.word	.Linfo_string54
	.word	3130
	.byte	8
	.byte	0
	.byte	0
	.byte	23
	.byte	4
	.word	.Linfo_string59
	.word	3148
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string54
	.byte	24
	.byte	1
	.byte	8
	.byte	14
	.word	5596
	.word	.Linfo_string21
	.byte	0
	.byte	16
	.word	.Linfo_string59
	.byte	24
	.byte	1
	.byte	8
	.byte	14
	.word	5596
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string60
	.word	5596
	.byte	8
	.byte	0
	.byte	1
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string129
	.byte	32
	.byte	1
	.byte	8
	.byte	18
	.word	3192
	.byte	19
	.word	4816
	.byte	8
	.byte	0

	.byte	20
	.byte	2
	.byte	4
	.word	.Linfo_string54
	.word	3227
	.byte	8
	.byte	0
	.byte	0
	.byte	23
	.byte	4
	.word	.Linfo_string59
	.word	3245
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string54
	.byte	32
	.byte	1
	.byte	8
	.byte	14
	.word	706
	.word	.Linfo_string21
	.byte	0
	.byte	16
	.word	.Linfo_string59
	.byte	32
	.byte	1
	.byte	8
	.byte	14
	.word	706
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string60
	.word	706
	.byte	8
	.byte	0
	.byte	1
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string132
	.byte	8
	.byte	1
	.byte	4
	.byte	18
	.word	3289
	.byte	19
	.word	6052
	.byte	4
	.byte	0

	.byte	20
	.byte	0
	.byte	4
	.word	.Linfo_string54
	.word	3325
	.byte	4
	.byte	0
	.byte	0
	.byte	20
	.byte	1
	.byte	4
	.word	.Linfo_string59
	.word	3343
	.byte	4
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string54
	.byte	8
	.byte	1
	.byte	4
	.byte	14
	.word	6052
	.word	.Linfo_string21
	.byte	0
	.byte	16
	.word	.Linfo_string59
	.byte	8
	.byte	1
	.byte	4
	.byte	14
	.word	6052
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string60
	.word	6052
	.byte	4
	.byte	4
	.byte	1
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string225
	.byte	16
	.byte	1
	.byte	8
	.byte	18
	.word	3387
	.byte	19
	.word	4816
	.byte	8
	.byte	0

	.byte	20
	.byte	0
	.byte	4
	.word	.Linfo_string54
	.word	3422
	.byte	8
	.byte	0
	.byte	0
	.byte	23
	.byte	4
	.word	.Linfo_string59
	.word	3440
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string54
	.byte	16
	.byte	1
	.byte	8
	.byte	14
	.word	6195
	.word	.Linfo_string21
	.byte	0
	.byte	16
	.word	.Linfo_string59
	.byte	16
	.byte	1
	.byte	8
	.byte	14
	.word	6195
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string60
	.word	6195
	.byte	8
	.byte	0
	.byte	1
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string230
	.byte	16
	.byte	1
	.byte	8
	.byte	18
	.word	3484
	.byte	19
	.word	4816
	.byte	8
	.byte	0

	.byte	20
	.byte	0
	.byte	4
	.word	.Linfo_string54
	.word	3520
	.byte	8
	.byte	0
	.byte	0
	.byte	20
	.byte	1
	.byte	4
	.word	.Linfo_string59
	.word	3538
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string54
	.byte	16
	.byte	1
	.byte	8
	.byte	14
	.word	159
	.word	.Linfo_string21
	.byte	0
	.byte	16
	.word	.Linfo_string59
	.byte	16
	.byte	1
	.byte	8
	.byte	14
	.word	159
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string60
	.word	159
	.byte	8
	.byte	8
	.byte	1
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string62
	.byte	16
	.word	.Linfo_string66
	.byte	16
	.byte	1
	.byte	8
	.byte	17
	.word	.Linfo_string63
	.word	5429
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string92
	.byte	43
	.word	6025

	.word	.Linfo_string96
	.byte	1
	.byte	1
	.byte	44
	.word	.Linfo_string94
	.byte	0
	.byte	44
	.word	.Linfo_string95
	.byte	1
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string76
	.byte	7
	.word	.Linfo_string152
	.byte	16
	.word	.Linfo_string157
	.byte	4
	.byte	1
	.byte	4
	.byte	17
	.word	.Linfo_string153
	.word	3665
	.byte	4
	.byte	0
	.byte	3
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string154
	.byte	16
	.word	.Linfo_string156
	.byte	4
	.byte	1
	.byte	4
	.byte	14
	.word	6052
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string155
	.word	6052
	.byte	4
	.byte	0
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string166
	.byte	32
	.byte	1
	.byte	8
	.byte	14
	.word	891
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string155
	.word	891
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string160
	.byte	7
	.word	.Linfo_string161
	.byte	16
	.word	.Linfo_string162
	.byte	32
	.byte	1
	.byte	8
	.byte	14
	.word	538
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string155
	.word	538
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string164
	.byte	32
	.byte	1
	.byte	8
	.byte	14
	.word	808
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string155
	.word	808
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string180
	.byte	7
	.word	.Linfo_string14
	.byte	43
	.word	6025

	.word	.Linfo_string185
	.byte	1
	.byte	1
	.byte	44
	.word	.Linfo_string181
	.byte	0
	.byte	44
	.word	.Linfo_string182
	.byte	1
	.byte	44
	.word	.Linfo_string183
	.byte	2
	.byte	44
	.word	.Linfo_string184
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string223
	.byte	56
	.byte	1
	.byte	8
	.byte	17
	.word	.Linfo_string213
	.word	159
	.byte	8
	.byte	32
	.byte	1
	.byte	17
	.word	.Linfo_string214
	.word	6234
	.byte	4
	.byte	40
	.byte	1
	.byte	17
	.word	.Linfo_string9
	.word	3808
	.byte	1
	.byte	48
	.byte	1
	.byte	17
	.word	.Linfo_string216
	.word	6052
	.byte	4
	.byte	44
	.byte	1
	.byte	17
	.word	.Linfo_string217
	.word	3925
	.byte	8
	.byte	0
	.byte	1
	.byte	17
	.word	.Linfo_string222
	.word	3925
	.byte	8
	.byte	16
	.byte	1
	.byte	0
	.byte	16
	.word	.Linfo_string221
	.byte	16
	.byte	1
	.byte	8
	.byte	18
	.word	3938
	.byte	19
	.word	4816
	.byte	8
	.byte	0

	.byte	20
	.byte	0
	.byte	4
	.word	.Linfo_string218
	.word	3988
	.byte	8
	.byte	0
	.byte	0
	.byte	20
	.byte	1
	.byte	4
	.word	.Linfo_string219
	.word	4009
	.byte	8
	.byte	0
	.byte	0
	.byte	20
	.byte	2
	.byte	4
	.word	.Linfo_string220
	.word	4030
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string218
	.byte	16
	.byte	1
	.byte	8
	.byte	17
	.word	.Linfo_string60
	.word	159
	.byte	8
	.byte	8
	.byte	1
	.byte	0
	.byte	16
	.word	.Linfo_string219
	.byte	16
	.byte	1
	.byte	8
	.byte	17
	.word	.Linfo_string60
	.word	159
	.byte	8
	.byte	8
	.byte	1
	.byte	0
	.byte	21
	.word	.Linfo_string220
	.byte	16
	.byte	1
	.byte	8
	.byte	0
	.byte	16
	.word	.Linfo_string239
	.byte	16
	.byte	1
	.byte	8
	.byte	17
	.word	.Linfo_string227
	.word	4125
	.byte	8
	.byte	0
	.byte	3
	.byte	38
	.word	.Linfo_string246
	.word	.Linfo_string247
	.byte	10
	.byte	117
	.word	4039

	.byte	14
	.word	4828
	.word	.Linfo_string21
	.byte	27
	.word	6478
	.byte	0
	.byte	38
	.word	.Linfo_string258
	.word	.Linfo_string259
	.byte	10
	.byte	99
	.word	4039

	.byte	14
	.word	4828
	.word	.Linfo_string21
	.byte	27
	.word	6478
	.byte	27
	.word	6558
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string238
	.byte	16
	.byte	3
	.byte	8
	.byte	18
	.word	4138
	.byte	19
	.word	4816
	.byte	8
	.byte	0

	.byte	23
	.byte	4
	.word	.Linfo_string223
	.word	4173
	.byte	8
	.byte	0
	.byte	0
	.byte	20
	.byte	0
	.byte	4
	.word	.Linfo_string221
	.word	4218
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string223
	.byte	16
	.byte	3
	.byte	8
	.byte	17
	.word	.Linfo_string155
	.word	2482
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.word	.Linfo_string228
	.word	6280
	.byte	8
	.byte	8
	.byte	3
	.byte	17
	.word	.Linfo_string237
	.word	2785
	.byte	1
	.byte	16
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string221
	.byte	16
	.byte	3
	.byte	8
	.byte	17
	.word	.Linfo_string60
	.word	159
	.byte	8
	.byte	8
	.byte	3
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string241
	.byte	48
	.byte	1
	.byte	8
	.byte	17
	.word	.Linfo_string208
	.word	6117
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.word	.Linfo_string180
	.word	3374
	.byte	8
	.byte	32
	.byte	3
	.byte	17
	.word	.Linfo_string226
	.word	6241
	.byte	8
	.byte	16
	.byte	3
	.byte	26
	.word	.Linfo_string242
	.word	.Linfo_string243
	.byte	9
	.half	348
	.word	4241

	.byte	27
	.word	6394
	.byte	27
	.word	6420
	.byte	0
	.byte	0
	.byte	21
	.word	.Linfo_string174
	.byte	0
	.byte	1
	.byte	1
	.byte	16
	.word	.Linfo_string234
	.byte	64
	.byte	1
	.byte	8
	.byte	17
	.word	.Linfo_string216
	.word	6052
	.byte	4
	.byte	52
	.byte	3
	.byte	17
	.word	.Linfo_string214
	.word	6234
	.byte	4
	.byte	48
	.byte	3
	.byte	17
	.word	.Linfo_string9
	.word	3808
	.byte	1
	.byte	56
	.byte	3
	.byte	17
	.word	.Linfo_string222
	.word	3471
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.word	.Linfo_string217
	.word	3471
	.byte	8
	.byte	16
	.byte	3
	.byte	17
	.word	.Linfo_string106
	.word	6322
	.byte	8
	.byte	32
	.byte	3
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string199
	.byte	45
	.word	.Linfo_string200
	.word	.Linfo_string201
	.byte	6
	.half	388
	.byte	1
	.byte	14
	.word	152
	.word	.Linfo_string21
	.byte	46
	.word	.Linfo_string202
	.byte	6
	.half	388
	.word	152
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string203
	.byte	7
	.word	.Linfo_string204
	.byte	7
	.word	.Linfo_string205
	.byte	9
	.quad	.Lfunc_begin4
	.word	.Lfunc_end4-.Lfunc_begin4
	.byte	1
	.byte	82
	.word	.Linfo_string274
	.word	.Linfo_string275
	.byte	7
	.byte	250
	.word	6059
	.byte	47
	.byte	2
	.byte	145
	.byte	16
	.byte	7
	.byte	250
	.word	6710
	.byte	47
	.byte	2
	.byte	145
	.byte	15
	.byte	7
	.byte	250
	.word	152
	.byte	14
	.word	181
	.word	.Linfo_string272
	.byte	14
	.word	152
	.word	.Linfo_string273
	.byte	0
	.byte	9
	.quad	.Lfunc_begin5
	.word	.Lfunc_end5-.Lfunc_begin5
	.byte	1
	.byte	82
	.word	.Linfo_string276
	.word	.Linfo_string277
	.byte	7
	.byte	250
	.word	1803
	.byte	47
	.byte	2
	.byte	145
	.byte	16
	.byte	7
	.byte	250
	.word	1775
	.byte	47
	.byte	2
	.byte	145
	.byte	15
	.byte	7
	.byte	250
	.word	152
	.byte	14
	.word	1775
	.word	.Linfo_string272
	.byte	14
	.word	152
	.word	.Linfo_string273
	.byte	0
	.byte	9
	.quad	.Lfunc_begin6
	.word	.Lfunc_end6-.Lfunc_begin6
	.byte	1
	.byte	82
	.word	.Linfo_string278
	.word	.Linfo_string275
	.byte	7
	.byte	250
	.word	6059
	.byte	47
	.byte	2
	.byte	145
	.byte	8
	.byte	7
	.byte	250
	.word	181
	.byte	47
	.byte	2
	.byte	145
	.byte	23
	.byte	7
	.byte	250
	.word	152
	.byte	14
	.word	181
	.word	.Linfo_string272
	.byte	14
	.word	152
	.word	.Linfo_string273
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string285
	.byte	16
	.word	.Linfo_string293
	.byte	8
	.byte	1
	.byte	8
	.byte	18
	.word	4683
	.byte	19
	.word	4816
	.byte	8
	.byte	0

	.byte	20
	.byte	0
	.byte	4
	.word	.Linfo_string286
	.word	4718
	.byte	8
	.byte	0
	.byte	0
	.byte	23
	.byte	4
	.word	.Linfo_string292
	.word	4757
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.word	.Linfo_string286
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.word	2296
	.word	.Linfo_string290
	.byte	14
	.word	152
	.word	.Linfo_string291
	.byte	17
	.word	.Linfo_string60
	.word	152
	.byte	1
	.byte	0
	.byte	1
	.byte	0
	.byte	16
	.word	.Linfo_string292
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.word	2296
	.word	.Linfo_string290
	.byte	14
	.word	152
	.word	.Linfo_string291
	.byte	17
	.word	.Linfo_string60
	.word	2296
	.byte	8
	.byte	0
	.byte	1
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string287
	.byte	16
	.word	.Linfo_string288
	.byte	0
	.byte	1
	.byte	1
	.byte	25
	.byte	0
	.byte	0
	.byte	0
	.byte	6
	.word	.Linfo_string19
	.byte	7
	.byte	8
	.byte	7
	.word	.Linfo_string22
	.byte	16
	.word	.Linfo_string174
	.byte	8
	.byte	1
	.byte	8
	.byte	17
	.word	.Linfo_string23
	.word	4854
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	7
	.word	.Linfo_string24
	.byte	16
	.word	.Linfo_string173
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.word	5016
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string24
	.word	2452
	.byte	8
	.byte	0
	.byte	1
	.byte	0
	.byte	16
	.word	.Linfo_string42
	.byte	16
	.byte	1
	.byte	8
	.byte	14
	.word	5198
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string24
	.word	2422
	.byte	8
	.byte	0
	.byte	1
	.byte	17
	.word	.Linfo_string38
	.word	2749
	.byte	1
	.byte	16
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string47
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.word	5016
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string24
	.word	2452
	.byte	8
	.byte	0
	.byte	1
	.byte	17
	.word	.Linfo_string38
	.word	2767
	.byte	1
	.byte	8
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string58
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.word	152
	.word	.Linfo_string21
	.byte	17
	.word	.Linfo_string24
	.word	2482
	.byte	8
	.byte	0
	.byte	1
	.byte	17
	.word	.Linfo_string38
	.word	2785
	.byte	1
	.byte	8
	.byte	3
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string25
	.byte	16
	.word	.Linfo_string172
	.byte	56
	.byte	2
	.byte	8
	.byte	14
	.word	152
	.word	.Linfo_string26
	.byte	17
	.word	.Linfo_string27
	.word	5141
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.word	.Linfo_string72
	.word	2978
	.byte	8
	.byte	8
	.byte	3
	.byte	17
	.word	.Linfo_string171
	.word	152
	.byte	1
	.byte	56
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string70
	.byte	40
	.byte	3
	.byte	8
	.byte	17
	.word	.Linfo_string28
	.word	5154
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.word	.Linfo_string30
	.word	5174
	.byte	8
	.byte	8
	.byte	3
	.byte	17
	.word	.Linfo_string49
	.word	5333
	.byte	8
	.byte	16
	.byte	3
	.byte	17
	.word	.Linfo_string52
	.word	5387
	.byte	8
	.byte	24
	.byte	3
	.byte	17
	.word	.Linfo_string68
	.word	5459
	.byte	8
	.byte	32
	.byte	3
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.word	5070
	.word	.Linfo_string71
	.word	0
	.byte	5
	.word	5167
	.word	.Linfo_string29
	.word	0
	.byte	48
	.byte	27
	.word	4854
	.byte	0
	.byte	5
	.word	5187
	.word	.Linfo_string48
	.word	0
	.byte	49
	.word	4884
	.byte	27
	.word	4926
	.byte	0
	.byte	50
	.word	.Linfo_string31
	.byte	0
	.byte	1
	.byte	8
	.word	.Linfo_string36
	.byte	16
	.byte	8
	.byte	4
	.word	.Linfo_string33
	.word	5235
	.byte	8
	.byte	0
	.byte	4
	.word	.Linfo_string27
	.word	5244
	.byte	8
	.byte	8
	.byte	0
	.byte	51
	.word	5198
	.word	0
	.byte	5
	.word	5257
	.word	.Linfo_string35
	.word	0
	.byte	52
	.word	159
	.byte	53
	.word	5270
	.byte	0
	.byte	10
	.byte	0
	.byte	54
	.word	.Linfo_string34
	.byte	8
	.byte	7
	.byte	8
	.word	.Linfo_string40
	.byte	16
	.byte	8
	.byte	4
	.word	.Linfo_string33
	.word	5235
	.byte	8
	.byte	0
	.byte	4
	.word	.Linfo_string27
	.word	5244
	.byte	8
	.byte	8
	.byte	0
	.byte	5
	.word	5016
	.word	.Linfo_string43
	.word	0
	.byte	5
	.word	5016
	.word	.Linfo_string45
	.word	0
	.byte	5
	.word	5346
	.word	.Linfo_string51
	.word	0
	.byte	49
	.word	5357
	.byte	27
	.word	4854
	.byte	0
	.byte	8
	.word	.Linfo_string50
	.byte	16
	.byte	8
	.byte	4
	.word	.Linfo_string33
	.word	5235
	.byte	8
	.byte	0
	.byte	4
	.word	.Linfo_string27
	.word	5244
	.byte	8
	.byte	8
	.byte	0
	.byte	5
	.word	5400
	.word	.Linfo_string67
	.word	0
	.byte	49
	.word	2881
	.byte	27
	.word	4926
	.byte	27
	.word	3575
	.byte	0
	.byte	5
	.word	152
	.word	.Linfo_string56
	.word	0
	.byte	8
	.word	.Linfo_string65
	.byte	16
	.byte	8
	.byte	4
	.word	.Linfo_string60
	.word	4816
	.byte	8
	.byte	0
	.byte	4
	.word	.Linfo_string64
	.word	4816
	.byte	8
	.byte	8
	.byte	0
	.byte	5
	.word	5472
	.word	.Linfo_string69
	.word	0
	.byte	48
	.byte	27
	.word	4854
	.byte	27
	.word	3575
	.byte	0
	.byte	7
	.word	.Linfo_string80
	.byte	7
	.word	.Linfo_string81
	.byte	16
	.word	.Linfo_string141
	.byte	24
	.byte	1
	.byte	8
	.byte	14
	.word	571
	.word	.Linfo_string21
	.byte	14
	.word	5704
	.word	.Linfo_string105
	.byte	17
	.word	.Linfo_string106
	.word	5946
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.word	.Linfo_string118
	.word	159
	.byte	8
	.byte	16
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string137
	.byte	24
	.byte	1
	.byte	8
	.byte	14
	.word	649
	.word	.Linfo_string21
	.byte	14
	.word	5704
	.word	.Linfo_string105
	.byte	17
	.word	.Linfo_string106
	.word	5895
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.word	.Linfo_string118
	.word	159
	.byte	8
	.byte	16
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string119
	.byte	24
	.byte	1
	.byte	8
	.byte	14
	.word	6025
	.word	.Linfo_string21
	.byte	14
	.word	5704
	.word	.Linfo_string105
	.byte	17
	.word	.Linfo_string106
	.word	5718
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.word	.Linfo_string118
	.word	159
	.byte	8
	.byte	16
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string127
	.byte	24
	.byte	1
	.byte	8
	.byte	14
	.word	6045
	.word	.Linfo_string21
	.byte	14
	.word	5704
	.word	.Linfo_string105
	.byte	17
	.word	.Linfo_string106
	.word	5844
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.word	.Linfo_string118
	.word	159
	.byte	8
	.byte	16
	.byte	3
	.byte	0
	.byte	0
	.byte	7
	.word	.Linfo_string80
	.byte	21
	.word	.Linfo_string104
	.byte	0
	.byte	1
	.byte	1
	.byte	0
	.byte	7
	.word	.Linfo_string107
	.byte	16
	.word	.Linfo_string117
	.byte	16
	.byte	2
	.byte	8
	.byte	14
	.word	6025
	.word	.Linfo_string21
	.byte	14
	.word	5704
	.word	.Linfo_string105
	.byte	17
	.word	.Linfo_string23
	.word	5769
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.word	.Linfo_string111
	.word	2803
	.byte	1
	.byte	16
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string116
	.byte	16
	.byte	3
	.byte	8
	.byte	14
	.word	5704
	.word	.Linfo_string105
	.byte	17
	.word	.Linfo_string24
	.word	2653
	.byte	8
	.byte	8
	.byte	3
	.byte	17
	.word	.Linfo_string114
	.word	5823
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.word	.Linfo_string80
	.word	5704
	.byte	1
	.byte	16
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string115
	.byte	8
	.byte	3
	.byte	8
	.byte	17
	.word	.Linfo_string60
	.word	159
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string126
	.byte	16
	.byte	2
	.byte	8
	.byte	14
	.word	6045
	.word	.Linfo_string21
	.byte	14
	.word	5704
	.word	.Linfo_string105
	.byte	17
	.word	.Linfo_string23
	.word	5769
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.word	.Linfo_string111
	.word	2821
	.byte	1
	.byte	16
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string136
	.byte	16
	.byte	2
	.byte	8
	.byte	14
	.word	649
	.word	.Linfo_string21
	.byte	14
	.word	5704
	.word	.Linfo_string105
	.byte	17
	.word	.Linfo_string23
	.word	5769
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.word	.Linfo_string111
	.word	2839
	.byte	1
	.byte	16
	.byte	3
	.byte	0
	.byte	16
	.word	.Linfo_string140
	.byte	16
	.byte	2
	.byte	8
	.byte	14
	.word	571
	.word	.Linfo_string21
	.byte	14
	.word	5704
	.word	.Linfo_string105
	.byte	17
	.word	.Linfo_string23
	.word	5769
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.word	.Linfo_string111
	.word	2857
	.byte	1
	.byte	16
	.byte	3
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.word	1125
	.word	.Linfo_string89
	.word	0
	.byte	5
	.word	3602
	.word	.Linfo_string97
	.word	0
	.byte	6
	.word	.Linfo_string93
	.byte	7
	.byte	1
	.byte	5
	.word	6025
	.word	.Linfo_string109
	.word	0
	.byte	6
	.word	.Linfo_string124
	.byte	7
	.byte	2
	.byte	6
	.word	.Linfo_string131
	.byte	7
	.byte	4
	.byte	6
	.word	.Linfo_string194
	.byte	5
	.byte	4
	.byte	55
	.word	1402
	.byte	1
	.byte	40
	.word	.Linfo_string195
	.byte	3
	.half	2052
	.word	1382
	.byte	0
	.byte	5
	.word	1203
	.word	.Linfo_string198
	.word	0
	.byte	55
	.word	1223
	.byte	1
	.byte	40
	.word	.Linfo_string195
	.byte	4
	.half	635
	.word	6085
	.byte	0
	.byte	8
	.word	.Linfo_string212
	.byte	16
	.byte	8
	.byte	4
	.word	.Linfo_string209
	.word	6147
	.byte	8
	.byte	0
	.byte	4
	.word	.Linfo_string210
	.word	159
	.byte	8
	.byte	8
	.byte	0
	.byte	51
	.word	6156
	.word	0
	.byte	8
	.word	.Linfo_string211
	.byte	16
	.byte	8
	.byte	4
	.word	.Linfo_string209
	.word	6186
	.byte	8
	.byte	0
	.byte	4
	.word	.Linfo_string210
	.word	159
	.byte	8
	.byte	8
	.byte	0
	.byte	51
	.word	6025
	.word	0
	.byte	8
	.word	.Linfo_string224
	.byte	16
	.byte	8
	.byte	4
	.word	.Linfo_string209
	.word	6225
	.byte	8
	.byte	0
	.byte	4
	.word	.Linfo_string210
	.word	159
	.byte	8
	.byte	8
	.byte	0
	.byte	51
	.word	3844
	.word	0
	.byte	6
	.word	.Linfo_string215
	.byte	16
	.byte	4
	.byte	8
	.word	.Linfo_string240
	.byte	16
	.byte	8
	.byte	4
	.word	.Linfo_string209
	.word	6271
	.byte	8
	.byte	0
	.byte	4
	.word	.Linfo_string210
	.word	159
	.byte	8
	.byte	8
	.byte	0
	.byte	51
	.word	4039
	.word	0
	.byte	5
	.word	6293
	.word	.Linfo_string236
	.word	0
	.byte	49
	.word	2168
	.byte	27
	.word	2482
	.byte	27
	.word	6309
	.byte	0
	.byte	5
	.word	4321
	.word	.Linfo_string235
	.word	0
	.byte	8
	.word	.Linfo_string233
	.byte	16
	.byte	8
	.byte	4
	.word	.Linfo_string33
	.word	6352
	.byte	8
	.byte	0
	.byte	4
	.word	.Linfo_string27
	.word	6368
	.byte	8
	.byte	8
	.byte	0
	.byte	51
	.word	6361
	.word	0
	.byte	50
	.word	.Linfo_string231
	.byte	0
	.byte	1
	.byte	5
	.word	6381
	.word	.Linfo_string232
	.word	0
	.byte	52
	.word	159
	.byte	53
	.word	5270
	.byte	0
	.byte	6
	.byte	0
	.byte	5
	.word	6407
	.word	.Linfo_string244
	.word	0
	.byte	52
	.word	6156
	.byte	53
	.word	5270
	.byte	0
	.byte	2
	.byte	0
	.byte	5
	.word	6433
	.word	.Linfo_string245
	.word	0
	.byte	52
	.word	4039
	.byte	53
	.word	5270
	.byte	0
	.byte	1
	.byte	0
	.byte	55
	.word	4285
	.byte	1
	.byte	40
	.word	.Linfo_string226
	.byte	9
	.half	350
	.word	6420
	.byte	56
	.word	.Linfo_string208
	.byte	1
	.byte	9
	.half	349
	.word	6394
	.byte	0
	.byte	5
	.word	4828
	.word	.Linfo_string248
	.word	0
	.byte	55
	.word	4059
	.byte	1
	.byte	14
	.word	4828
	.word	.Linfo_string21
	.byte	57
	.word	.Linfo_string249
	.byte	10
	.byte	117
	.word	6478
	.byte	0
	.byte	5
	.word	4828
	.word	.Linfo_string250
	.word	0
	.byte	55
	.word	2571
	.byte	1
	.byte	14
	.word	4828
	.word	.Linfo_string21
	.byte	57
	.word	.Linfo_string254
	.byte	11
	.byte	232
	.word	6478
	.byte	0
	.byte	5
	.word	6571
	.word	.Linfo_string260
	.word	0
	.byte	49
	.word	2168
	.byte	27
	.word	6478
	.byte	27
	.word	6309
	.byte	0
	.byte	55
	.word	4089
	.byte	1
	.byte	14
	.word	4828
	.word	.Linfo_string21
	.byte	57
	.word	.Linfo_string249
	.byte	10
	.byte	99
	.word	6478
	.byte	57
	.word	.Linfo_string163
	.byte	10
	.byte	99
	.word	6558
	.byte	0
	.byte	7
	.word	.Linfo_string262
	.byte	58
	.quad	.Lfunc_begin11
	.word	.Lfunc_end11-.Lfunc_begin11
	.byte	1
	.byte	82
	.word	.Linfo_string296
	.word	.Linfo_string16
	.byte	12
	.byte	25
	.word	1803

	.byte	28
	.quad	.Ltmp56
	.word	.Ltmp57-.Ltmp56
	.byte	10
	.byte	2
	.byte	145
	.byte	32
	.word	.Linfo_string297
	.byte	1
	.byte	12
	.byte	37
	.word	2296
	.byte	0
	.byte	0
	.byte	0
	.byte	6
	.word	.Linfo_string267
	.byte	5
	.byte	8
	.byte	5
	.word	6032
	.word	.Linfo_string301
	.word	0
	.byte	5
	.word	181
	.word	.Linfo_string303
	.word	0
	.byte	0
.Ldebug_info_end0:
	.section	.data.rel.ro..L__unnamed_1,"aw",@progbits
.Lsec_end0:
	.section	".text._ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h3ac5837e8ed948e2E","ax",@progbits
.Lsec_end1:
	.section	.text._ZN3std2rt10lang_start17h62fb49f3140921d1E,"ax",@progbits
.Lsec_end2:
	.section	".text._ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h5d866f0ed9f04cd9E","ax",@progbits
.Lsec_end3:
	.section	.text._ZN3std3sys9backtrace28__rust_begin_short_backtrace17h678179e6fd03b5aaE,"ax",@progbits
.Lsec_end4:
	.section	".text._ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he4f6803a51943c29E","ax",@progbits
.Lsec_end5:
	.section	.text._ZN4core3ops8function6FnOnce9call_once17h70ee1792a0f8136aE,"ax",@progbits
.Lsec_end6:
	.section	.text._ZN4core3ops8function6FnOnce9call_once17hdb553d2527f36714E,"ax",@progbits
.Lsec_end7:
	.section	".text._ZN4core3ptr129drop_in_place$LT$std..rt..lang_start$LT$core..result..Result$LT$$LP$$RP$$C$anyhow..Error$GT$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h558dd7f1a2a5f091E","ax",@progbits
.Lsec_end8:
	.section	".text._ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hb37c1fcd02863739E","ax",@progbits
.Lsec_end9:
	.section	".text._ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h8c8033d2b111120eE","ax",@progbits
.Lsec_end10:
	.section	".text._ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$std..process..Termination$GT$6report17h7710fdf22934ad1eE","ax",@progbits
.Lsec_end11:
	.section	.text._ZN16proof_generation4main17h92c5237958f168d4E,"ax",@progbits
.Lsec_end12:
	.section	.debug_aranges,"",@progbits
	.word	236
	.half	2
	.word	.Lcu_begin0
	.byte	8
	.byte	0
	.zero	4,255
	.quad	.L__unnamed_1
	.quad	.Lsec_end0-.L__unnamed_1
	.quad	.Lfunc_begin0
	.quad	.Lsec_end1-.Lfunc_begin0
	.quad	.Lfunc_begin1
	.quad	.Lsec_end2-.Lfunc_begin1
	.quad	.Lfunc_begin2
	.quad	.Lsec_end3-.Lfunc_begin2
	.quad	.Lfunc_begin3
	.quad	.Lsec_end4-.Lfunc_begin3
	.quad	.Lfunc_begin4
	.quad	.Lsec_end5-.Lfunc_begin4
	.quad	.Lfunc_begin5
	.quad	.Lsec_end6-.Lfunc_begin5
	.quad	.Lfunc_begin6
	.quad	.Lsec_end7-.Lfunc_begin6
	.quad	.Lfunc_begin7
	.quad	.Lsec_end8-.Lfunc_begin7
	.quad	.Lfunc_begin8
	.quad	.Lsec_end9-.Lfunc_begin8
	.quad	.Lfunc_begin9
	.quad	.Lsec_end10-.Lfunc_begin9
	.quad	.Lfunc_begin10
	.quad	.Lsec_end11-.Lfunc_begin10
	.quad	.Lfunc_begin11
	.quad	.Lsec_end12-.Lfunc_begin11
	.quad	0
	.quad	0
	.section	.debug_ranges,"",@progbits
.Ldebug_ranges0:
	.quad	.Ltmp40
	.quad	.Ltmp41
	.quad	.Ltmp44
	.quad	.Ltmp51
	.quad	.Ltmp52
	.quad	.Ltmp53
	.quad	0
	.quad	0
.Ldebug_ranges1:
	.quad	.Ltmp40
	.quad	.Ltmp41
	.quad	.Ltmp49
	.quad	.Ltmp50
	.quad	0
	.quad	0
.Ldebug_ranges2:
	.quad	.Lfunc_begin0
	.quad	.Lfunc_end0
	.quad	.Lfunc_begin1
	.quad	.Lfunc_end1
	.quad	.Lfunc_begin2
	.quad	.Lfunc_end2
	.quad	.Lfunc_begin3
	.quad	.Lfunc_end3
	.quad	.Lfunc_begin4
	.quad	.Lfunc_end4
	.quad	.Lfunc_begin5
	.quad	.Lfunc_end5
	.quad	.Lfunc_begin6
	.quad	.Lfunc_end6
	.quad	.Lfunc_begin7
	.quad	.Lfunc_end7
	.quad	.Lfunc_begin8
	.quad	.Lfunc_end8
	.quad	.Lfunc_begin9
	.quad	.Lfunc_end9
	.quad	.Lfunc_begin10
	.quad	.Lfunc_end10
	.quad	.Lfunc_begin11
	.quad	.Lfunc_end11
	.quad	0
	.quad	0
	.section	.debug_str,"MS",@progbits,1
.Linfo_string0:
	.asciz	"clang LLVM (rustc version 1.84.0-nightly (bc5cf994d 2024-11-05))"
.Linfo_string1:
	.asciz	"proof_generation/src/main.rs/@/67lu8i1mr7ampavgpxd68b557"
.Linfo_string2:
	.asciz	"/home/alirezza/Documents/Projects/OtherProjects/Fides/zkpRust/zkIoT-Rust"
.Linfo_string3:
	.asciz	"<std::rt::lang_start::{closure_env#0}<core::result::Result<(), anyhow::Error>> as core::ops::function::Fn<()>>::{vtable}"
.Linfo_string4:
	.asciz	"drop_in_place"
.Linfo_string5:
	.asciz	"()"
.Linfo_string6:
	.asciz	"*const ()"
.Linfo_string7:
	.asciz	"size"
.Linfo_string8:
	.asciz	"usize"
.Linfo_string9:
	.asciz	"align"
.Linfo_string10:
	.asciz	"__method3"
.Linfo_string11:
	.asciz	"__method4"
.Linfo_string12:
	.asciz	"__method5"
.Linfo_string13:
	.asciz	"std"
.Linfo_string14:
	.asciz	"rt"
.Linfo_string15:
	.asciz	"lang_start"
.Linfo_string16:
	.asciz	"main"
.Linfo_string17:
	.asciz	"core"
.Linfo_string18:
	.asciz	"result"
.Linfo_string19:
	.asciz	"u64"
.Linfo_string20:
	.asciz	"Ok"
.Linfo_string21:
	.asciz	"T"
.Linfo_string22:
	.asciz	"anyhow"
.Linfo_string23:
	.asciz	"inner"
.Linfo_string24:
	.asciz	"ptr"
.Linfo_string25:
	.asciz	"error"
.Linfo_string26:
	.asciz	"E"
.Linfo_string27:
	.asciz	"vtable"
.Linfo_string28:
	.asciz	"object_drop"
.Linfo_string29:
	.asciz	"unsafe fn(anyhow::ptr::Own<anyhow::error::ErrorImpl<()>>)"
.Linfo_string30:
	.asciz	"object_ref"
.Linfo_string31:
	.asciz	"(dyn core::error::Error + core::marker::Send + core::marker::Sync)"
.Linfo_string32:
	.asciz	"non_null"
.Linfo_string33:
	.asciz	"pointer"
.Linfo_string34:
	.asciz	"__ARRAY_SIZE_TYPE__"
.Linfo_string35:
	.asciz	"&[usize; 10]"
.Linfo_string36:
	.asciz	"*const (dyn core::error::Error + core::marker::Send + core::marker::Sync)"
.Linfo_string37:
	.asciz	"NonNull<(dyn core::error::Error + core::marker::Send + core::marker::Sync)>"
.Linfo_string38:
	.asciz	"lifetime"
.Linfo_string39:
	.asciz	"marker"
.Linfo_string40:
	.asciz	"&(dyn core::error::Error + core::marker::Send + core::marker::Sync)"
.Linfo_string41:
	.asciz	"PhantomData<&(dyn core::error::Error + core::marker::Send + core::marker::Sync)>"
.Linfo_string42:
	.asciz	"Ref<(dyn core::error::Error + core::marker::Send + core::marker::Sync)>"
.Linfo_string43:
	.asciz	"*const anyhow::error::ErrorImpl<()>"
.Linfo_string44:
	.asciz	"NonNull<anyhow::error::ErrorImpl<()>>"
.Linfo_string45:
	.asciz	"&anyhow::error::ErrorImpl<()>"
.Linfo_string46:
	.asciz	"PhantomData<&anyhow::error::ErrorImpl<()>>"
.Linfo_string47:
	.asciz	"Ref<anyhow::error::ErrorImpl<()>>"
.Linfo_string48:
	.asciz	"unsafe fn(anyhow::ptr::Ref<anyhow::error::ErrorImpl<()>>) -> anyhow::ptr::Ref<(dyn core::error::Error + core::marker::Send + core::marker::Sync)>"
.Linfo_string49:
	.asciz	"object_boxed"
.Linfo_string50:
	.asciz	"alloc::boxed::Box<(dyn core::error::Error + core::marker::Send + core::marker::Sync), alloc::alloc::Global>"
.Linfo_string51:
	.asciz	"unsafe fn(anyhow::ptr::Own<anyhow::error::ErrorImpl<()>>) -> alloc::boxed::Box<(dyn core::error::Error + core::marker::Send + core::marker::Sync), alloc::alloc::Global>"
.Linfo_string52:
	.asciz	"object_downcast"
.Linfo_string53:
	.asciz	"option"
.Linfo_string54:
	.asciz	"None"
.Linfo_string55:
	.asciz	"NonNull<()>"
.Linfo_string56:
	.asciz	"&()"
.Linfo_string57:
	.asciz	"PhantomData<&()>"
.Linfo_string58:
	.asciz	"Ref<()>"
.Linfo_string59:
	.asciz	"Some"
.Linfo_string60:
	.asciz	"__0"
.Linfo_string61:
	.asciz	"Option<anyhow::ptr::Ref<()>>"
.Linfo_string62:
	.asciz	"any"
.Linfo_string63:
	.asciz	"t"
.Linfo_string64:
	.asciz	"__1"
.Linfo_string65:
	.asciz	"(u64, u64)"
.Linfo_string66:
	.asciz	"TypeId"
.Linfo_string67:
	.asciz	"unsafe fn(anyhow::ptr::Ref<anyhow::error::ErrorImpl<()>>, core::any::TypeId) -> core::option::Option<anyhow::ptr::Ref<()>>"
.Linfo_string68:
	.asciz	"object_drop_rest"
.Linfo_string69:
	.asciz	"unsafe fn(anyhow::ptr::Own<anyhow::error::ErrorImpl<()>>, core::any::TypeId)"
.Linfo_string70:
	.asciz	"ErrorVTable"
.Linfo_string71:
	.asciz	"&anyhow::error::ErrorVTable"
.Linfo_string72:
	.asciz	"backtrace"
.Linfo_string73:
	.asciz	"Unsupported"
.Linfo_string74:
	.asciz	"Disabled"
.Linfo_string75:
	.asciz	"Captured"
.Linfo_string76:
	.asciz	"sync"
.Linfo_string77:
	.asciz	"lazy_lock"
.Linfo_string78:
	.asciz	"actual_start"
.Linfo_string79:
	.asciz	"frames"
.Linfo_string80:
	.asciz	"alloc"
.Linfo_string81:
	.asciz	"vec"
.Linfo_string82:
	.asciz	"frame"
.Linfo_string83:
	.asciz	"Actual"
.Linfo_string84:
	.asciz	"backtrace_rs"
.Linfo_string85:
	.asciz	"libunwind"
.Linfo_string86:
	.asciz	"Raw"
.Linfo_string87:
	.asciz	"uw"
.Linfo_string88:
	.asciz	"_Unwind_Context"
.Linfo_string89:
	.asciz	"*mut std::backtrace_rs::backtrace::libunwind::uw::_Unwind_Context"
.Linfo_string90:
	.asciz	"Cloned"
.Linfo_string91:
	.asciz	"ip"
.Linfo_string92:
	.asciz	"ffi"
.Linfo_string93:
	.asciz	"u8"
.Linfo_string94:
	.asciz	"__variant1"
.Linfo_string95:
	.asciz	"__variant2"
.Linfo_string96:
	.asciz	"c_void"
.Linfo_string97:
	.asciz	"*mut core::ffi::c_void"
.Linfo_string98:
	.asciz	"sp"
.Linfo_string99:
	.asciz	"symbol_address"
.Linfo_string100:
	.asciz	"Frame"
.Linfo_string101:
	.asciz	"RawFrame"
.Linfo_string102:
	.asciz	"symbols"
.Linfo_string103:
	.asciz	"name"
.Linfo_string104:
	.asciz	"Global"
.Linfo_string105:
	.asciz	"A"
.Linfo_string106:
	.asciz	"buf"
.Linfo_string107:
	.asciz	"raw_vec"
.Linfo_string108:
	.asciz	"unique"
.Linfo_string109:
	.asciz	"*const u8"
.Linfo_string110:
	.asciz	"NonNull<u8>"
.Linfo_string111:
	.asciz	"_marker"
.Linfo_string112:
	.asciz	"PhantomData<u8>"
.Linfo_string113:
	.asciz	"Unique<u8>"
.Linfo_string114:
	.asciz	"cap"
.Linfo_string115:
	.asciz	"Cap"
.Linfo_string116:
	.asciz	"RawVecInner<alloc::alloc::Global>"
.Linfo_string117:
	.asciz	"RawVec<u8, alloc::alloc::Global>"
.Linfo_string118:
	.asciz	"len"
.Linfo_string119:
	.asciz	"Vec<u8, alloc::alloc::Global>"
.Linfo_string120:
	.asciz	"Option<alloc::vec::Vec<u8, alloc::alloc::Global>>"
.Linfo_string121:
	.asciz	"filename"
.Linfo_string122:
	.asciz	"Bytes"
.Linfo_string123:
	.asciz	"Wide"
.Linfo_string124:
	.asciz	"u16"
.Linfo_string125:
	.asciz	"PhantomData<u16>"
.Linfo_string126:
	.asciz	"RawVec<u16, alloc::alloc::Global>"
.Linfo_string127:
	.asciz	"Vec<u16, alloc::alloc::Global>"
.Linfo_string128:
	.asciz	"BytesOrWide"
.Linfo_string129:
	.asciz	"Option<std::backtrace::BytesOrWide>"
.Linfo_string130:
	.asciz	"lineno"
.Linfo_string131:
	.asciz	"u32"
.Linfo_string132:
	.asciz	"Option<u32>"
.Linfo_string133:
	.asciz	"colno"
.Linfo_string134:
	.asciz	"BacktraceSymbol"
.Linfo_string135:
	.asciz	"PhantomData<std::backtrace::BacktraceSymbol>"
.Linfo_string136:
	.asciz	"RawVec<std::backtrace::BacktraceSymbol, alloc::alloc::Global>"
.Linfo_string137:
	.asciz	"Vec<std::backtrace::BacktraceSymbol, alloc::alloc::Global>"
.Linfo_string138:
	.asciz	"BacktraceFrame"
.Linfo_string139:
	.asciz	"PhantomData<std::backtrace::BacktraceFrame>"
.Linfo_string140:
	.asciz	"RawVec<std::backtrace::BacktraceFrame, alloc::alloc::Global>"
.Linfo_string141:
	.asciz	"Vec<std::backtrace::BacktraceFrame, alloc::alloc::Global>"
.Linfo_string142:
	.asciz	"Capture"
.Linfo_string143:
	.asciz	"helper"
.Linfo_string144:
	.asciz	"lazy_resolve"
.Linfo_string145:
	.asciz	"capture"
.Linfo_string146:
	.asciz	"{closure_env#0}"
.Linfo_string147:
	.asciz	"F"
.Linfo_string148:
	.asciz	"once"
.Linfo_string149:
	.asciz	"sys"
.Linfo_string150:
	.asciz	"futex"
.Linfo_string151:
	.asciz	"state_and_queued"
.Linfo_string152:
	.asciz	"atomic"
.Linfo_string153:
	.asciz	"v"
.Linfo_string154:
	.asciz	"cell"
.Linfo_string155:
	.asciz	"value"
.Linfo_string156:
	.asciz	"UnsafeCell<u32>"
.Linfo_string157:
	.asciz	"AtomicU32"
.Linfo_string158:
	.asciz	"Once"
.Linfo_string159:
	.asciz	"data"
.Linfo_string160:
	.asciz	"mem"
.Linfo_string161:
	.asciz	"manually_drop"
.Linfo_string162:
	.asciz	"ManuallyDrop<std::backtrace::Capture>"
.Linfo_string163:
	.asciz	"f"
.Linfo_string164:
	.asciz	"ManuallyDrop<std::backtrace::helper::lazy_resolve::{closure_env#0}>"
.Linfo_string165:
	.asciz	"Data<std::backtrace::Capture, std::backtrace::helper::lazy_resolve::{closure_env#0}>"
.Linfo_string166:
	.asciz	"UnsafeCell<std::sync::lazy_lock::Data<std::backtrace::Capture, std::backtrace::helper::lazy_resolve::{closure_env#0}>>"
.Linfo_string167:
	.asciz	"LazyLock<std::backtrace::Capture, std::backtrace::helper::lazy_resolve::{closure_env#0}>"
.Linfo_string168:
	.asciz	"Inner"
.Linfo_string169:
	.asciz	"Backtrace"
.Linfo_string170:
	.asciz	"Option<std::backtrace::Backtrace>"
.Linfo_string171:
	.asciz	"_object"
.Linfo_string172:
	.asciz	"ErrorImpl<()>"
.Linfo_string173:
	.asciz	"Own<anyhow::error::ErrorImpl<()>>"
.Linfo_string174:
	.asciz	"Error"
.Linfo_string175:
	.asciz	"Err"
.Linfo_string176:
	.asciz	"Result<(), anyhow::Error>"
.Linfo_string177:
	.asciz	"fn() -> core::result::Result<(), anyhow::Error>"
.Linfo_string178:
	.asciz	"{closure_env#0}<core::result::Result<(), anyhow::Error>>"
.Linfo_string179:
	.asciz	"<std::rt::lang_start::{closure_env#0}<core::result::Result<(), anyhow::Error>> as core::ops::function::Fn<()>>::{vtable_type}"
.Linfo_string180:
	.asciz	"fmt"
.Linfo_string181:
	.asciz	"Left"
.Linfo_string182:
	.asciz	"Right"
.Linfo_string183:
	.asciz	"Center"
.Linfo_string184:
	.asciz	"Unknown"
.Linfo_string185:
	.asciz	"Alignment"
.Linfo_string186:
	.asciz	"{impl#27}"
.Linfo_string187:
	.asciz	"process"
.Linfo_string188:
	.asciz	"pal"
.Linfo_string189:
	.asciz	"unix"
.Linfo_string190:
	.asciz	"process_common"
.Linfo_string191:
	.asciz	"ExitCode"
.Linfo_string192:
	.asciz	"_ZN3std7process8ExitCode6to_i3217h1043ee719f9030feE"
.Linfo_string193:
	.asciz	"to_i32"
.Linfo_string194:
	.asciz	"i32"
.Linfo_string195:
	.asciz	"self"
.Linfo_string196:
	.asciz	"_ZN3std3sys3pal4unix7process14process_common8ExitCode6as_i3217h691639cdb36f6504E"
.Linfo_string197:
	.asciz	"as_i32"
.Linfo_string198:
	.asciz	"&std::sys::pal::unix::process::process_common::ExitCode"
.Linfo_string199:
	.asciz	"hint"
.Linfo_string200:
	.asciz	"_ZN4core4hint9black_box17h70db8adff76fab9aE"
.Linfo_string201:
	.asciz	"black_box<()>"
.Linfo_string202:
	.asciz	"dummy"
.Linfo_string203:
	.asciz	"ops"
.Linfo_string204:
	.asciz	"function"
.Linfo_string205:
	.asciz	"FnOnce"
.Linfo_string206:
	.asciz	"{impl#57}"
.Linfo_string207:
	.asciz	"{impl#26}"
.Linfo_string208:
	.asciz	"pieces"
.Linfo_string209:
	.asciz	"data_ptr"
.Linfo_string210:
	.asciz	"length"
.Linfo_string211:
	.asciz	"&str"
.Linfo_string212:
	.asciz	"&[&str]"
.Linfo_string213:
	.asciz	"position"
.Linfo_string214:
	.asciz	"fill"
.Linfo_string215:
	.asciz	"char"
.Linfo_string216:
	.asciz	"flags"
.Linfo_string217:
	.asciz	"precision"
.Linfo_string218:
	.asciz	"Is"
.Linfo_string219:
	.asciz	"Param"
.Linfo_string220:
	.asciz	"Implied"
.Linfo_string221:
	.asciz	"Count"
.Linfo_string222:
	.asciz	"width"
.Linfo_string223:
	.asciz	"Placeholder"
.Linfo_string224:
	.asciz	"&[core::fmt::rt::Placeholder]"
.Linfo_string225:
	.asciz	"Option<&[core::fmt::rt::Placeholder]>"
.Linfo_string226:
	.asciz	"args"
.Linfo_string227:
	.asciz	"ty"
.Linfo_string228:
	.asciz	"formatter"
.Linfo_string229:
	.asciz	"Result<(), core::fmt::Error>"
.Linfo_string230:
	.asciz	"Option<usize>"
.Linfo_string231:
	.asciz	"dyn core::fmt::Write"
.Linfo_string232:
	.asciz	"&[usize; 6]"
.Linfo_string233:
	.asciz	"&mut dyn core::fmt::Write"
.Linfo_string234:
	.asciz	"Formatter"
.Linfo_string235:
	.asciz	"&mut core::fmt::Formatter"
.Linfo_string236:
	.asciz	"unsafe fn(core::ptr::non_null::NonNull<()>, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>"
.Linfo_string237:
	.asciz	"_lifetime"
.Linfo_string238:
	.asciz	"ArgumentType"
.Linfo_string239:
	.asciz	"Argument"
.Linfo_string240:
	.asciz	"&[core::fmt::rt::Argument]"
.Linfo_string241:
	.asciz	"Arguments"
.Linfo_string242:
	.asciz	"_ZN4core3fmt9Arguments6new_v117h87c518bf26564df9E"
.Linfo_string243:
	.asciz	"new_v1<2, 1>"
.Linfo_string244:
	.asciz	"&[&str; 2]"
.Linfo_string245:
	.asciz	"&[core::fmt::rt::Argument; 1]"
.Linfo_string246:
	.asciz	"_ZN4core3fmt2rt8Argument9new_debug17h61bf916e74d0fb88E"
.Linfo_string247:
	.asciz	"new_debug<anyhow::Error>"
.Linfo_string248:
	.asciz	"&anyhow::Error"
.Linfo_string249:
	.asciz	"x"
.Linfo_string250:
	.asciz	"*const anyhow::Error"
.Linfo_string251:
	.asciz	"NonNull<anyhow::Error>"
.Linfo_string252:
	.asciz	"_ZN4core3ptr8non_null16NonNull$LT$T$GT$8from_ref17h684ad27939f5de3dE"
.Linfo_string253:
	.asciz	"from_ref<anyhow::Error>"
.Linfo_string254:
	.asciz	"r"
.Linfo_string255:
	.asciz	"{impl#19}"
.Linfo_string256:
	.asciz	"_ZN90_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$$RF$T$GT$$GT$4from17h5d2742e38b00b84bE"
.Linfo_string257:
	.asciz	"from<anyhow::Error>"
.Linfo_string258:
	.asciz	"_ZN4core3fmt2rt8Argument3new17h2ba24dc27b960b94E"
.Linfo_string259:
	.asciz	"new<anyhow::Error>"
.Linfo_string260:
	.asciz	"fn(&anyhow::Error, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>"
.Linfo_string261:
	.asciz	"{impl#61}"
.Linfo_string262:
	.asciz	"proof_generation"
.Linfo_string263:
	.asciz	"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h3ac5837e8ed948e2E"
.Linfo_string264:
	.asciz	"from_residual<(), anyhow::Error, anyhow::Error>"
.Linfo_string265:
	.asciz	"_ZN3std2rt10lang_start17h62fb49f3140921d1E"
.Linfo_string266:
	.asciz	"lang_start<core::result::Result<(), anyhow::Error>>"
.Linfo_string267:
	.asciz	"isize"
.Linfo_string268:
	.asciz	"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h5d866f0ed9f04cd9E"
.Linfo_string269:
	.asciz	"{closure#0}<core::result::Result<(), anyhow::Error>>"
.Linfo_string270:
	.asciz	"_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h678179e6fd03b5aaE"
.Linfo_string271:
	.asciz	"__rust_begin_short_backtrace<fn() -> core::result::Result<(), anyhow::Error>, core::result::Result<(), anyhow::Error>>"
.Linfo_string272:
	.asciz	"Self"
.Linfo_string273:
	.asciz	"Args"
.Linfo_string274:
	.asciz	"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he4f6803a51943c29E"
.Linfo_string275:
	.asciz	"call_once<std::rt::lang_start::{closure_env#0}<core::result::Result<(), anyhow::Error>>, ()>"
.Linfo_string276:
	.asciz	"_ZN4core3ops8function6FnOnce9call_once17h70ee1792a0f8136aE"
.Linfo_string277:
	.asciz	"call_once<fn() -> core::result::Result<(), anyhow::Error>, ()>"
.Linfo_string278:
	.asciz	"_ZN4core3ops8function6FnOnce9call_once17hdb553d2527f36714E"
.Linfo_string279:
	.asciz	"_ZN4core3ptr129drop_in_place$LT$std..rt..lang_start$LT$core..result..Result$LT$$LP$$RP$$C$anyhow..Error$GT$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h558dd7f1a2a5f091E"
.Linfo_string280:
	.asciz	"drop_in_place<std::rt::lang_start::{closure_env#0}<core::result::Result<(), anyhow::Error>>>"
.Linfo_string281:
	.asciz	"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hb37c1fcd02863739E"
.Linfo_string282:
	.asciz	"report"
.Linfo_string283:
	.asciz	"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h8c8033d2b111120eE"
.Linfo_string284:
	.asciz	"branch<(), anyhow::Error>"
.Linfo_string285:
	.asciz	"control_flow"
.Linfo_string286:
	.asciz	"Continue"
.Linfo_string287:
	.asciz	"convert"
.Linfo_string288:
	.asciz	"Infallible"
.Linfo_string289:
	.asciz	"Result<core::convert::Infallible, anyhow::Error>"
.Linfo_string290:
	.asciz	"B"
.Linfo_string291:
	.asciz	"C"
.Linfo_string292:
	.asciz	"Break"
.Linfo_string293:
	.asciz	"ControlFlow<core::result::Result<core::convert::Infallible, anyhow::Error>, ()>"
.Linfo_string294:
	.asciz	"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$std..process..Termination$GT$6report17h7710fdf22934ad1eE"
.Linfo_string295:
	.asciz	"report<(), anyhow::Error>"
.Linfo_string296:
	.asciz	"_ZN16proof_generation4main17h92c5237958f168d4E"
.Linfo_string297:
	.asciz	"residual"
.Linfo_string298:
	.asciz	"e"
.Linfo_string299:
	.asciz	"argc"
.Linfo_string300:
	.asciz	"argv"
.Linfo_string301:
	.asciz	"*const *const u8"
.Linfo_string302:
	.asciz	"sigpipe"
.Linfo_string303:
	.asciz	"*mut std::rt::lang_start::{closure_env#0}<core::result::Result<(), anyhow::Error>>"
.Linfo_string304:
	.asciz	"val"
.Linfo_string305:
	.asciz	"err"
	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"awG",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3, 0x0
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality
	.ident	"rustc version 1.84.0-nightly (bc5cf994d 2024-11-05)"
	.section	".note.GNU-stack","",@progbits
	.section	.debug_line,"",@progbits
.Lline_table_start0:
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
x9_array:    .space 4   # Array for x9
x10_array:    .space 4   # Array for x10
x11_array:    .space 4   # Array for x11
x12_array:    .space 4   # Array for x12
x13_array:    .space 4   # Array for x13
x14_array:    .space 4   # Array for x14
x15_array:    .space 4   # Array for x15
x16_array:    .space 4   # Array for x16
x17_array:    .space 4   # Array for x17
x18_array:    .space 20   # Array for x18
x19_array:    .space 4   # Array for x19
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
    sw x0, x0_array(0)            # Store x0 in x0_array at index given by 0
    sw x1, x1_array(0)            # Store x1 in x1_array at index given by 0
    sw x2, x2_array(0)            # Store x2 in x2_array at index given by 0
    sw x3, x3_array(0)            # Store x3 in x3_array at index given by 0
    sw x4, x4_array(0)            # Store x4 in x4_array at index given by 0
    sw x5, x5_array(0)            # Store x5 in x5_array at index given by 0
    sw x6, x6_array(0)            # Store x6 in x6_array at index given by 0
    sw x7, x7_array(0)            # Store x7 in x7_array at index given by 0
    sw x8, x8_array(0)            # Store x8 in x8_array at index given by 0
    sw x9, x9_array(0)            # Store x9 in x9_array at index given by 0
    sw x10, x10_array(0)          # Store x10 in x10_array at index given by 0
    sw x11, x11_array(0)          # Store x11 in x11_array at index given by 0
    sw x12, x12_array(0)          # Store x12 in x12_array at index given by 0
    sw x13, x13_array(0)          # Store x13 in x13_array at index given by 0
    sw x14, x14_array(0)          # Store x14 in x14_array at index given by 0
    sw x15, x15_array(0)          # Store x15 in x15_array at index given by 0
    sw x16, x16_array(0)          # Store x16 in x16_array at index given by 0
    sw x17, x17_array(0)          # Store x17 in x17_array at index given by 0
    sw x18, x18_array(0)          # Store x18 in x18_array at index given by 0
    sw x19, x19_array(0)          # Store x19 in x19_array at index given by 0
    sw x20, x20_array(0)          # Store x20 in x20 array at index given by 0
    sw x21, x21_array(0)          # Store x21 in x21_array at index given by 0
    sw x22, x22_array(0)          # Store x22 in x22_array at index given by 0
    sw x23, x23_array(0)          # Store x23 in x23_array at index given by 0
    sw x24, x24_array(0)          # Store x24 in x24_array at index given by 0
    sw x25, x25_array(0)          # Store x25 in x25_array at index given by 0
    sw x26, x26_array(0)          # Store x26 in x26_array at index given by 0
    sw x27, x27_array(0)          # Store x27 in x27_array at index given by 0
    sw x28, x28_array(0)          # Store x28 in x28_array at index given by 0
    sw x29, x29_array(0)          # Store x29 in x29_array at index given by 0
    sw x30, x30_array(0)          # Store x30 in x30_array at index given by 0
    sw x31, x31_array(0)          # Store x31 in x31_array at index given by 0

    ret                            # Return from function
