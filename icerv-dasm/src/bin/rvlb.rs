//────────────────────────────────────────────────
//  RVLB: RISC-V LIST BYTES
//    Ejecutable para visualizar un fichero binario
//    en bytes
//────────────────────────────────────────────────
use std::{fs::File, io::Read};
use icerv_dasm::ansi;
use icerv_dasm::graphics::lineh;



fn dump16(addr: u32, buf: &Vec<u8>, index: usize) -> usize
//────────────────────────────────────────────────
//  Hacer un volcado de una línea de 16 bytes
//
//  ENTRADAS:
//    - addr: Direccion del volcado
//    - buf: Buffer con los bytes a volcar
//    - index: byte actual con el que comenzar el volcado
//
//  SALIDA:
//    - Se devuelve la cantidad de bytes mostrados
//      * Linea completa: 16
//      * Linea no completa: <16
//────────────────────────────────────────────────
{
    let mut index: usize = index;

    //-- Contador de bytes volcados
    let mut n_bytes: usize = 0;

    //-- Imprimir la direccion actual
    print!("{}", ansi::YELLOW);
    print!("{:08X}  ", addr);
    print!("{}", ansi::RESET);

    //-- Imprimir una línea, byte a byte
    for _ in 0..16 {

        //-- Imprimir byte actual
        let byte = buf.get(index);
        match byte {
            Some(value) => {
                print!("{:02X} ", value);
                n_bytes += 1;
            }
            None => {
                //-- No hay más datos: Terminar
                //-- Linea no completa
                println!();
                return n_bytes;
            }
        }

        //-- Apuntar al siguiente byte
        index += 1;
    }

    println!();

    //-- Linea completa: 16 bytes volcados
    return 16;

}

fn dump(buf: &Vec<u8>) 
//────────────────────────────────────────────────
//  Realizar el volcado
//
//  ENTRADAS:
//    - buf: Buffer con los bytes a volcar
//────────────────────────────────────────────────
{
    //-- Indice en buffer: señala al byte actual
    let mut index: usize = 0;

    //-- Calcular la cantidad de líneas totales
    let lines = (buf.len() / 16) + 1;

    //-- Direccion actual. Se empieza en 0
    let mut addr: u32 = 0;

    //-- Escribir los offsets
    print!("{}", ansi::GREEN);
    println!("          00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
    
    //-- Volcar todas las líneas
    for _ in 0..lines {

        //-- Volcar una línea (16 bytes) y obtener la cantidad de bytes
        //-- volcados
        let tam = dump16(addr, &buf, index);

        //-- Incrementar la direccion
        addr += tam as u32;

        //-- Incrementar el índice
        index += tam;
    }
}

fn show_dump(buf: &Vec<u8>)
//────────────────────────────────────────────────
//  Mostrar el volcado completo
//────────────────────────────────────────────────
{
    //-- Escribir la cabecera
    println!("Bytes totales: {}", buf.len());
    print!("{}", ansi::BLUE);
    lineh(60);

    //-- Generar el volcado!
    dump(&buf);

    //-- Escribir el prólogo
    print!("{}", ansi::BLUE);
    lineh(60);

    //-- Reseteo de atributos
    println!("{}", ansi::RESET);
    println!();
}

fn main()  
{
     //-- Borrado de pantalla
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
            println!("  Uso: rvlb fichero");
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

    //-- Buffer donde leer el contenido del fichero
    let mut buffer: Vec<u8> = Vec::new();

    //-- Leer fichero
    file.read_to_end(&mut buffer)
        .expect("Error en la lectura!");

    //-- Mostrar el volcado
    show_dump(&buffer);

}

//-- Pruebas de volcado
#[test]
fn test1() 
{
    //────────────────────────────────────────────────
    //  Definir el vector con los bytes
    //────────────────────────────────────────────────
    let buf: Vec<u8> = vec![
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,

        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,

        0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
        0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E,
    ];

    //-- Mostrar el volcado
    show_dump(&buf);
}