//────────────────────────────────────────────────
//  rv-dasm.rs
//  Desensamblador de instrucciones RISC-V
//  Autor: Obijuan
//  Fecha: 09/07/2025
//  Licencia: CC BY-SA 4.0
//────────────────────────────────────────────────

//-- Instrucciones RV32I
//-- https://msyksphinz-self.github.io/riscv-isadoc/html/rvi.html

#[cfg(test)]
mod test;


//────────────────────────────────────────────────
//  MODULOS usados
//────────────────────────────────────────────────

//──────── Instrucciones completas del RiscV
pub mod instructionrv;

//──────── Registros del RiscV
pub mod regs;

//──────── Código máquina
mod mcode;

//──────── Códigos de operación
mod opcoderv;

//────────────────────────────────────────────────
//  PROGRAMA PRINCIPAL
//────────────────────────────────────────────────

use instructionrv::InstructionRV;

fn main() {

    //-- Progama a desensamblar
    let mcode = [
        0x00100093, //-- addi x1, x0, 1
        0x00111093, //-- slli x1, x2, 1
        0x00112093, //-- slti x1, x2, 1
        0x00113093, //-- sltiu x1, x2, 1
        0x00114093, //-- xori x1, x2, 1
        0x00115093, //-- srli x1, x2, 1
        0x00116093, //-- ori x1, x2, 1
        0x00117093, //-- andi x1, x2, 1
        0x40115093, //-- srai x1, x2, 1
    ];

    //-- Desensamblar programa instrucción a instrucción
    for i in 0..mcode.len() {

        //-- Leer instrucción a partir del código máquina
        let inst= InstructionRV::from_mcode(mcode[i]);

        //-- Imprimir instrucción en código máquina y en ensamblador
        println!("🟢 [{:#010X}]: {}", mcode[i] as u32, inst.to_string());
    }
}

