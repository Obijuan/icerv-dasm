#![allow(dead_code)]

use crate::opcoderv::OpcodeRV;

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

//────────────────────────────────────────────────
//  CALCULAR LAS MASCARAS DE ACCESO A LOS CAMPOS
//────────────────────────────────────────────────
const OPCODE_MASK: u32 = FIELD_7B << OPCODE_POS; 

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

}

#[test]
fn test_opcode() {
    assert_eq!(MCode::new(0x00000013).opcode() as u32, 0x13);
}
