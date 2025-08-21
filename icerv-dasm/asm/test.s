li	gp,2
nop
auipc	a0,0x2
addi	a0,a0,1820 # 0x2724
jal	a1,0x14
sub	a0,a0,a1
lui	t2,0x2
addi	t2,t2,1808 # 0x2710
bne	a0,t2,0x2c

