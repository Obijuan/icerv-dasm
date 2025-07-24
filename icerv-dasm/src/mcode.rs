#![allow(dead_code)]

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

    pub fn opcode(&self) -> u32 {
        (self.value & OPCODE_MASK) >> OPCODE_POS
    }

}

#[test]
fn test_opcode() {
    assert_eq!(MCode::new(0x00000013).opcode(), 0x13);
}
