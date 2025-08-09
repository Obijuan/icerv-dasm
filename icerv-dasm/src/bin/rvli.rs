//────────────────────────────────────────────────
//  RVLI: RISC-V LIST INSTRUCTIONS
//    Ejecutable para visualizar el código máquina
//    de un fichero
//
//  Se leen instrucciones de 32-bits, alineadas
//────────────────────────────────────────────────

use std::{fs::File, io::Read};
use icerv_dasm::ansi;
use icerv_dasm::graphics::lineh;

fn dump_insts(buf: &Vec<u32>)
{
    let mut addr: u32 = 0;

    println!("Instrucciones: {}", buf.len());

    print!("{}", ansi::BLUE);
    lineh(30);

    println!("  Addr    Inst");
    for i in buf {
        println!("{}{:08X}{} {:08X}",ansi::YELLOW, addr, ansi::RESET, i);
        addr += 4;
    }

    print!("{}", ansi::BLUE);
    lineh(30);
    println!("{}", ansi::RESET);
}

fn dump_file(fich: String)
{
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
    
    // Bucle para leer el archivo 4 bytes a la vez
    while file.read_exact(&mut buffer).is_ok() {
        // Convierte los 4 bytes a un entero de 32 bits sin signo (u32)
        // La instrucción en RISC-V es de 32 bits (4 bytes)
        let instr = u32::from_le_bytes(buffer);

        // Meter la instruccion en el buffer de instrucciones
        buffer_words.push(instr);
    }

    dump_insts(&buffer_words);
}

fn main()
{
    //-- Borrar la pantalla
    print!("{}", ansi::CLS);

    //-- Leer primer argumento
    let arg = std::env::args().nth(1);
    let fich = match arg {
        Some(value) => {
            value
        }
        None => {
            print!("{}", ansi::RED);
            println!("Error: Fichero a volcar NO especificado");
            print!("{}", ansi::RESET);
            println!("  Uso: rvli fichero");
            return;
        }
    };

    //-- Volcar fichero
    dump_file(fich);
}

#[test]
fn test() 
{
        let insts: Vec<u32> = vec![
        0x00100093,
        0x00100113,
        0x00100193,
        0x0000006F,
    ];

    //-- Borrar la pantalla
    print!("{}", ansi::CLS);

    dump_insts(&insts);
}