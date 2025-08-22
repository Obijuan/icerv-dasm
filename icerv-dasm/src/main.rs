//────────────────────────────────────────────────
//  rv-dasm.rs
//  Desensamblador de instrucciones RISC-V
//  Autor: Obijuan
//  Fecha: 09/07/2025
//  Licencia: CC BY-SA 4.0
//────────────────────────────────────────────────

//-- Instrucciones RV32I
//-- https://msyksphinz-self.github.io/riscv-isadoc/html/rvi.html

use std::{fs::File, io::Read};
use icerv_dasm::ansi;
use icerv_dasm::graphics::lineh;
use icerv_dasm::instructionrv::InstructionRV;

#[cfg(test)]
mod test;
mod test_sim;


//────────────────────────────────────────────────
//  PROGRAMA PRINCIPAL
//────────────────────────────────────────────────
fn main() {

    //-- Leer primer argumento
    let arg = std::env::args().nth(1);
    let fich = match arg {
        Some(value) => {
            value
        }
        None => {
            print!("{}", ansi::RED);
            println!("Error: Fichero a simular NO especificado");
            print!("{}", ansi::RESET);
            println!("  Uso: sim fichero");
            return;
        }
    };

    println!("{}{}{}",ansi::BLUE, fich, ansi::RESET);

    //-- Abrir fichero
    let ofile = File::open(fich);
    let mut file = match ofile {
        Ok(value) => {
            value
        }
        Err(error) => {
            println!("{}Error: {}{}", ansi::RED, error, ansi::RESET);
            println!();
            return
        }
    };

    //-- Crea un buffer para almacenar 4 bytes (32 bits)
    let mut buffer = [0; 4];

     //-- Buffer donde colocar las instrucciones
    let mut buffer_words: Vec<u32> = Vec::new();
    
    // Bucle para leer el archivo de 4 en 4 bytes
    while file.read_exact(&mut buffer).is_ok() {

        // Convierte los 4 bytes a un entero de 32 bits sin signo (u32)
        let instr = u32::from_le_bytes(buffer);

        // Meter la instruccion en el buffer de instrucciones
        buffer_words.push(instr);
    }

    let mut addr: u32 = 0;

    println!("Instrucciones: {}", buffer_words.len());

    print!("{}", ansi::BLUE);
    lineh(40);

    println!("  Addr    m.code     asm");
    for i in buffer_words {

        //-- Leer instrucción a partir del código máquina
        let inst= InstructionRV::from_mcode(i);

        //-- Imprimir la linea con Direccion, código máquina y asm
        println!("{}{:08X}{} {:08X}   {}{}",
            ansi::YELLOW, addr, ansi::RESET, i, ansi::GREEN, inst.to_string());

        //-- Siguiente direccion
        addr += 4;
    }

    print!("{}", ansi::BLUE);
    lineh(40);
    println!("{}", ansi::RESET);

}

#[test]
fn test() 
{
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