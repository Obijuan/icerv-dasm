#!/bin/bash

# -- Fichero a ensamblar
FICH="srai"


# -- Ensamblador
AS=riscv64-unknown-elf-as

# -- Conversor
OBJCOPY=riscv64-unknown-elf-objcopy

$AS -march=rv32i $FICH.s -o $FICH.o
$OBJCOPY -O binary $FICH.o $FICH.bin

