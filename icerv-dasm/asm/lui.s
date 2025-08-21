#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE LUI
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#───────────────────────────────────────────────────────────────────────
#         testnum, testreg, correctval,      code... 
#   TEST_CASE( 2, x1, 0x00000000,         lui x1, 0x00000 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 2

    lui x1, 0x00000  #-- code

    li  x7, 0x00000000  #-- correctval
    bne x1, x7, fail

#───────────────────────────────────────────────────────────────────────
#         testnum, testreg, correctval,      code... 
#   TEST_CASE( 3, x1, 0xfffff800, lui x1, 0xfffff;sra x1,x1,1);
#───────────────────────────────────────────────────────────────────────
    li  x3, 3

    lui x1, 0xfffff  #-- code
    sra x1,x1,1

    li  x7, 0xfffff800  #-- correctval
    bne x1, x7, fail


#───────────────────────────────────────────────────────────────────────
#         testnum, testreg, correctval,      code... 
#   TEST_CASE( 4, x1, 0x000007ff, lui x1, 0x7ffff;sra x1,x1,20);
#───────────────────────────────────────────────────────────────────────
    li  x3, 4

    lui x1, 0x7ffff #-- code
    sra x1, x1,20

    li  x7, 0x000007ff  #-- correctval
    bne x1, x7, fail

#───────────────────────────────────────────────────────────────────────
#         testnum, testreg, correctval,      code... 
#   TEST_CASE( 5, x1, 0xfffff800, lui x1, 0x80000;sra x1,x1,20);
#───────────────────────────────────────────────────────────────────────
    li  x3, 5

    lui x1, 0x80000
    sra x1,x1,20

    li  x7, 0xfffff800  #-- correctval
    bne x1, x7, fail

#───────────────────────────────────────────────────────────────────────
#         testnum, testreg, correctval,      code... 
#   TEST_CASE( 6, x0, 0, lui x0, 0x80000 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 5

    lui x0, 0x80000  #-- code

    li  x7, 0  #-- correctval
    bne x0, x7, fail


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

  #-------------------------------------------------------------
  # Basic tests
  #-------------------------------------------------------------

