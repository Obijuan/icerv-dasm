#![allow(dead_code)]


use crate::regs::Reg;
use crate::opcoderv::OpcodeRV;

//──────────────────────────────────────────────────────
//  DOCUMENTACION SOBRE EL FORMATO DEL RISC-V
//──────────────────────────────────────────────────────
//-- https://msyksphinz-self.github.io/riscv-isadoc/html/rvi.html

/* 
   TIPO I: Instrucciones aritméticas y de carga

   │31 30 29 28 27 26 25 24 23 22 21 20│19 18 17 16 15│14 13 12│11 10 9  8  7 │6  5  4  3  2  1  0 │
   ├──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┼──┴──┴──┴──┴──┼──┴──┴──┼──┴──┴──┴──┴──┼──┴──┴──┴──┴──┴──┴──┤
   │            imm12                  |     rs1      | func3  |      rd      |      opcode        |
   ╰───────────────────────────────────┴──────────────┴────────┴──────────────┴────────────────────╯ 
*/

//───────────────────────────
//  ANCHURAS de LOS CAMPOS
//───────────────────────────
const FIELD_1B: u32 = 0x01;
const FIELD_3B: u32 = 0x07;
const FIELD_4B: u32 = 0x0F;
const FIELD_5B: u32 = 0x1F;
const FIELD_6B: u32 = 0x3F;
const FIELD_7B: u32 = 0x7F;
const FIELD_8B: u32 = 0xFF;
const FIELD_10B: u32 = 0x3FF;
const FIELD_12B: u32 = 0xFFF;
const FIELD_20B: u32 = 0xFFFFF;

//────────────────────────────────────────────────
//  POSICIONES de LOS CAMPOS
//────────────────────────────────────────────────
const OPCODE_POS: u8 = 0;
const RD_POS: u8 = 7;  
const RS1_POS: u8 = 15;  

//────────────────────────────────────────────────
//  CALCULAR LAS MASCARAS DE ACCESO A LOS CAMPOS
//────────────────────────────────────────────────
const OPCODE_MASK: u32 = FIELD_7B << OPCODE_POS; 
const RD_MASK: u32 = FIELD_5B << RD_POS;  
const RS1_MASK: u32 = FIELD_5B << RS1_POS;  


//────────────────────────────────────────────────
//  Estructura para gestionar el codigo maquina
//────────────────────────────────────────────────    
pub struct MCode {

    //-- Código maquina
    pub value: u32,
}

impl MCode {

    //────────────────────────────────────────────────
    //  Construir una nueva instruccion en codigo maquina
    //────────────────────────────────────────────────    
    pub fn new(value: u32) -> Self {
        MCode { value }
    }

    //────────────────────────────────────────────────
    //  Obtener el opcode de la instruccion
    //────────────────────────────────────────────────    
    pub fn opcode(&self) -> OpcodeRV {
        let op = (self.value & OPCODE_MASK) >> OPCODE_POS;

        //-- Devolver el opcode como un valor del enum OpcodeRV
        match op {
          0b_00100_11 => OpcodeRV::TipoIArith,
          0b_00000_11 => OpcodeRV::TipoILoad,
          0b_01100_11 => OpcodeRV::TipoR,
          0b_01000_11 => OpcodeRV::TipoS,
          0b_11000_11 => OpcodeRV::TipoB,
          0b_01101_11 => OpcodeRV::TipoULui,
          0b_00101_11 => OpcodeRV::TipoUAuipc,
          0b_11011_11 => OpcodeRV::TipoJJal,
          0b_11001_11 => OpcodeRV::TipoJJalr,
          0b_11100_11 => OpcodeRV::TipoEcallEbreak,
          _ => OpcodeRV::Unknown,
        }
    }

    //────────────────────────────────────────────────
    //  Obtener el registro destino (rd) de la instrucción
    //────────────────────────────────────────────────
    pub fn rd(&self) -> Reg {
        let reg_num = (self.value & RD_MASK) >> RD_POS;
        Reg::new(reg_num as u8)
    }

    //────────────────────────────────────────────────
    //  Obtener el registro fuente 1 (rs1) de la instrucción
    //────────────────────────────────────────────────
    pub fn rs1(&self) -> Reg {
        let reg_num: u32 = (self.value & RS1_MASK) >> RS1_POS;
        Reg::new(reg_num as u8) 
    }
}



#[test]
fn test_opcode() {
    assert_eq!(MCode::new(0x00000013).opcode() as u32, 0x13);
    assert_eq!(MCode::new(0x00000093).opcode() as u32, 0x13);
}

#[test]
fn test_rd() {

    //-- Instrucciones reales
    assert_eq!(MCode::new(0x00000013).rd().to_str(), "x0");
    assert_eq!(MCode::new(0x00000093).rd().to_str(), "x1");
    assert_eq!(MCode::new(0x0aa00093).rd().to_str(), "x1");

    //-- Instrucciones inventandas
    assert_eq!(MCode::new(0x00000000 | 0b00000_0000000).rd().to_str(), "x0");
    assert_eq!(MCode::new(0x00000000 | 0b00001_0000000).rd().to_str(), "x1");
    assert_eq!(MCode::new(0xFFFF0000 | 0b00010_0000000).rd().to_str(), "x2");
    assert_eq!(MCode::new(0xFFFF0000 | 0b00100_0000000).rd().to_str(), "x4");
    assert_eq!(MCode::new(0xFFFF0000 | 0b01000_0000000).rd().to_str(), "x8");
    assert_eq!(MCode::new(0xFFFF0000 | 0b10000_0000000).rd().to_str(), "x16");
    assert_eq!(MCode::new(0xFFFF0000 | 0b10001_0000000).rd().to_str(), "x17");
    assert_eq!(MCode::new(0xFFFF0000 | 0b11111_0000000).rd().to_str(), "x31");
}   

#[test]
fn test_rs1() {

    //--                              func7  rs2   rs1  func3 rd    opcode
    let mcode = MCode::new(0b_0000000_00000_00000_000_00000_0000000);
    assert_eq!(mcode.rs1().to_str(), "x0");

    let mcode = MCode::new(0b_0000000_00000_00001_000_00000_0000000);
    assert_eq!(mcode.rs1().to_str(), "x1");

    let mcode = MCode::new(0b_0000000_00000_00010_000_00000_0000000);
    assert_eq!(mcode.rs1().to_str(), "x2");

    let mcode = MCode::new(0b_0000000_00000_00100_000_00000_0000000);
    assert_eq!(mcode.rs1().to_str(), "x4");

    let mcode = MCode::new(0b_0000000_00000_01000_000_00000_0000000);
    assert_eq!(mcode.rs1().to_str(), "x8");

    let mcode = MCode::new(0b_0000000_00000_10000_000_00000_0000000);
    assert_eq!(mcode.rs1().to_str(), "x16");

    let mcode = MCode::new(0b_0000000_00000_10001_000_00000_0000000);
    assert_eq!(mcode.rs1().to_str(), "x17");

    let mcode = MCode::new(0b_0000000_00000_11111_000_00000_0000000);
    assert_eq!(mcode.rs1().to_str(), "x31");

} 
    
