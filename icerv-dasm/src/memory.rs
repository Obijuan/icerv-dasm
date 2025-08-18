use std::{fs::File, io::Read};
use crate::{ansi};

//────────────────────────────────────────────────
//  MODELADO DE LA MEMORIA
//──────────────────────────────────────────────── 
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    //────────────────────────────────────────────────
    //  Constructor. Crear la memoria a partir de una
    //  lista de bytes
    //──────────────────────────────────────────────── 
    pub fn new(data: Vec<u8>) -> Self {
        Memory { data }
    }

    //────────────────────────────────────────────────
    //  Constructor: Crear la memoria a partir de un
    //  fichero
    //──────────────────────────────────────────────── 
    pub fn from_file(fich: &str) -> Self {

        //-- Abrir fichero
        let ofile = File::open(fich);
        let mut file = match ofile {
            Ok(value) => {
                value
            }
            Err(error) => {
                println!("{}Error: {}{}", ansi::RED, error, ansi::RESET);
                println!();
                let data:Vec<u8> = Vec::new();
                return Memory {data};
            }
        };

        //-- Buffer donde leer el contenido del fichero
        let mut data: Vec<u8> = Vec::new();


        //-- Leer fichero
        file.read_to_end(&mut data)
            .expect("Error en la lectura!");

        Memory { data }

    }

    //────────────────────────────────────────────────
    //  Devolver el tamaño de la memoria en bytes
    //──────────────────────────────────────────────── 
    pub fn size(&self) -> usize {
        self.data.len()
    }

    //────────────────────────────────────────────────
    //  Leer un byte
    //──────────────────────────────────────────────── 
    pub fn read8(&self, addr: u32) -> u8 {
        self.data[addr as usize]
    }

    //────────────────────────────────────────────────
    //  Leer una media palabra (16-bits)
    //──────────────────────────────────────────────── 
    pub fn read16(&self, addr: u32) -> u16 {

        // Asegura que la dirección esté alineada a 2 bytes
        assert!(addr % 2 == 0, "Dirección NO ALINEADA para media palabra.");

        //-- Buffer para almacenar los bytes de la memoria palabra
        let mut bytes = [0; 2];

        //-- Leer los dos bytes
        bytes[0] = self.data[addr as usize];
        bytes[1] = self.data[addr as usize +1];

        //bytes.copy_from_slice(&self.data[addr as usize..addr as usize + 2]);

        // Convierte los bytes a u16 usando little-endian
        u16::from_le_bytes(bytes)
    }

    //────────────────────────────────────────────────
    //  Leer una palabra (32-bits)
    //──────────────────────────────────────────────── 
    pub fn read32(&self, addr: u32) -> u32 {
        // Asegura que la dirección esté alineada a 4 bytes
        assert!(addr % 4 == 0, "Dirección no alineada para palabra.");

        //-- Buffer para almacenar los bytes de la palabra
        let mut bytes = [0; 4];

        bytes[0] = self.data[addr as usize];
        bytes[1] = self.data[addr as usize + 1];
        bytes[2] = self.data[addr as usize + 2];
        bytes[3] = self.data[addr as usize + 3];

        //bytes.copy_from_slice(&self.data[addr as usize..addr as usize + 4]);
        // Convierte los bytes a u32 usando little-endian
        u32::from_le_bytes(bytes)
    }

    //────────────────────────────────────────────────
    //  Escribir un byte
    //──────────────────────────────────────────────── 
    pub fn write8(&mut self, addr: u32, value: u8) {
        self.data[addr as usize]=value;
    }

    //────────────────────────────────────────────────
    //  Escribir una media palabra
    //──────────────────────────────────────────────── 
    pub fn write16(&mut self, addr: u32, value: u16) {

        assert!(addr % 2 == 0, "Dirección no alineada para media-palabra.");

        //-- Buffer para almacenar los bytes de la memoria palabra
        let bytes = value.to_le_bytes(); 

        self.data[addr as usize] = bytes[0];
        self.data[addr as usize + 1] = bytes[1];
    }
    //────────────────────────────────────────────────
    //  Escribir una palabra
    //──────────────────────────────────────────────── 
    pub fn write32(&mut self, addr: u32, value: u32) {

        assert!(addr % 4 == 0, "Dirección no alineada para palabra.");

        //-- Buffer para almacenar los bytes de la memoria palabra
        let bytes = value.to_le_bytes(); 

        self.data[addr as usize] = bytes[0];
        self.data[addr as usize + 1] = bytes[1];
        self.data[addr as usize + 2] = bytes[2];
        self.data[addr as usize + 3] = bytes[3];
    }
}

#[test]
fn test_mem_read_byte() {
    
    //-- Definir la memoria mediante bytes
    let data:Vec<u8> = vec![0x0, 0x10, 0x20, 0x30];
    let mem:Memory = Memory::new(data);

    //-- Tamaño de la memoria
    println!("Tamaño memoria: {}", mem.size());

    //-- Recorrer la memoria e imprimir sus bytes
    for addr in 0..mem.data.len() {
        println!("* [{addr}]: {:X}", mem.read8(addr as u32));
    }

    println!("* Media Palabra 1: {:X}", mem.read16(0));
    println!("* Media Palabra 2: {:X}", mem.read16(2));
   
    println!("* Palabra: {:X}", mem.read32(0));

}

#[test]
fn test2() {
    let data:Vec<u8> = vec![0; 20];
    let mut mem:Memory = Memory::new(data);

    println!("* Tamaño: {}", mem.size());

    for addr in 0..mem.size() {
        println!("[{}]: {}", addr, mem.read8(addr as u32));
    }

    mem.write8(0, 10);
    mem.write8(17, 20);
    mem.write16(2, 0xCAFE);
    mem.write32(8, 0xCAFEBACA);

    println!();
    for addr in 0..mem.size() {
        println!("[{}]: {:X}", addr, mem.read8(addr as u32));
    }
}

#[test]
fn test3() {
    let mem = Memory::from_file("asm/addi.bin");

    println!("* Tamaño: {}", mem.size());

    for addr in 0..mem.size() {
        println!("[{:#X}]: {:#X}", addr, mem.read8(addr as u32));
    }
}

#[test]
fn test4() {
    let mem = Memory::from_file("asm/lb.bin");
    let insts: u32 = (mem.size()>>2) as u32;

    println!("* Instrucciones: {}", insts);

    //-- Direccio actual
    let mut addr:u32 = 0;

    //-- Recorrer la memoria
    while addr < mem.size() as u32 {

        //-- Imprimir instruccion actual
        println!("* [{:#010X}]: {:#010X}", addr, mem.read32(addr));

        //-- Apuntar a la siguiente instruccion
        addr += 4;
    }
}

