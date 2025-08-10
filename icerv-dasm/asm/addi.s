#-- Pruebas para addi
#           testnum,  inst,  result,       val1,     imm
#    TEST_IMM_OP( 2,  addi, 0x00000000, 0x00000000, 0x000 );
#   x14 = val1 + imm --> x14 = result?
    li x3, 2  #-- testnum
    li x13, 0x00000000  #-- val1
    addi x14, x13, 0x00 #-- imm (testreg)
    li x7, 0x00000000 #-- result
    bne x14, x7, fail;

    #-- Test OK
    #-- x1=1
    #-- x3=Nº de test
pass:
    li x1, 1
    j .


    #-- Test NO pasado
    #-- x1=0
    #-- x3= Nº de test
fail:
    li x1, 0
    j .