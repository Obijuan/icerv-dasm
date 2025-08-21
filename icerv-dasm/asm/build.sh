#!/bin/bash

# -- Fichero a ensamblar
FICH="ecall"

# -- Ensamblador
AS=riscv64-unknown-elf-as

# -- Conversor
OBJCOPY=riscv64-unknown-elf-objcopy

#-- Ensamblar
$AS -march=rv32i $FICH.s -o $FICH.o

riscv64-unknown-elf-ld -T linker.ld  -m elf32lriscv -e 00000000 $FICH.o -o $FICH.out

#-- Generar ejecutable binario
$OBJCOPY -O binary  $FICH.out $FICH.bin

riscv64-unknown-elf-objdump -b binary -m riscv:rv32 -D $FICH.bin
