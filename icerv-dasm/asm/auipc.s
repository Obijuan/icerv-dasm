#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE AUIPC
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#───────────────────────────────────────────────────────────────────────
#   testnum, testreg, correctval, code... ) 
#   TEST_CASE(2, a0, 10000, 
#     .align 3
#     lla a0, 1f + 10000
#     jal a1, 1f
#     1: sub a0, a0, a1
#───────────────────────────────────────────────────────────────────────
    li  x3, 2  #-- testnum

    .align 3
    lla a0, 1f + 10000
    jal a1, 1f
1: sub a0, a0, a1

    li  x7, 10000
    bne a0, x7, fail


#───────────────────────────────────────────────────────────────────────
#   testnum, testreg, correctval, code... ) 
#   TEST_CASE(3, a0, -10000, 
#     .align 3
#     lla a0, 1f - 10000
#     jal a1, 1f
#     1: sub a0, a0, a1
#───────────────────────────────────────────────────────────────────────
    li  x3, 3  #-- testnum

    .align 3
    lla a0, 1f + 10000
    jal a1, 1f
1: sub a0, a0, a1

    li  x7, 10000
    bne a0, x7, fail


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

