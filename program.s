	#APP
	mul 	s2, s2, s3
	addi 	s2, s2, 11
	mul 	s2, s2, s4
	mul 	s2, s2, s4
	addi	ra, sp, gp
	addi	sp, gp, tp
	mul		s6, ra, t0
	addi	ra, sp, sp
	#NOAPP