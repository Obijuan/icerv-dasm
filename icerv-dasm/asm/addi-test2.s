#────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 2
#────────────────────────────────────────────────

    #-- Numero de test
    li x3, 2

    #-- Valor 1
    li x13, 0x00000000

    #-- Resultado obtenido: Sumar valor 1  y valor inmediato
    addi x14, x13, 0x00

    #-- x7: Valor del resultado esperado
    li x7, 0x00000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
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

