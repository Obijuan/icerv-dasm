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

    TIPO R

   │31 30 29 28 27 26 25│24 23 22 21 20│19 18 17 16 15│14 13 12│11 10 9  8  7 │6  5  4  3  2  1  0 │
   ├──┴──┴──┴──┴──┴──┴──┼──┴──┴──┴──┴──┼──┴──┴──┴──┴──┼──┴──┴──┼──┴──┴──┴──┴──┼──┴──┴──┴──┴──┴──┴──┤
   │       func7        │   rs2        |     rs1      | func3  |      rd      |      opcode        |
   ╰────────────────────┴──────────────┴──────────────┴────────┴──────────────┴────────────────────╯ 

   TIPO S

   │31 30 29 28 27 26 25│24 23 22 21 20│19 18 17 16 15│14 13 12│11 10 9  8  7 │6  5  4  3  2  1  0 │
   ├──┴──┴──┴──┴──┴──┴──┼──┴──┴──┴──┴──┼──┴──┴──┴──┴──┼──┴──┴──┼──┴──┴──┴──┴──┼──┴──┴──┴──┴──┴──┴──┤
   │   offset[11:5]     │   rs2        |     rs1      | func3  | offset[4:0]  |      opcode        |
   ╰────────────────────┴──────────────┴──────────────┴────────┴──────────────┴────────────────────╯    

    TIPO B

   │31 30 29 28 27 26 25│24 23 22 21 20│19 18 17 16 15│14 13 12│11 10 9  8  7 │6  5  4  3  2  1  0 │
   ├──┴──┴──┴──┴──┴──┴──┼──┴──┴──┴──┴──┼──┴──┴──┴──┴──┼──┴──┴──┼──┴──┴──┴──┴──┼──┴──┴──┴──┴──┴──┴──┤
   │  offset[12|10:5]   │   rs2        |     rs1      | func3  |offset[4:1|11]|      opcode        |
   ╰────────────────────┴──────────────┴──────────────┴────────┴──────────────┴────────────────────╯     

    TIPO U

   │31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12│11 10 9  8  7 │6  5  4  3  2  1  0 │
   ├──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┼──┴──┴──┴──┴──┼──┴──┴──┴──┴──┴──┴──┤
   │                        imm[31:12]                         |      rd      |      opcode        |
   ╰───────────────────────────────────────────────────────────┴──────────────┴────────────────────╯  

    TIPO J

   │31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12│11 10 9  8  7 │6  5  4  3  2  1  0 │
   ├──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┼──┴──┴──┴──┴──┼──┴──┴──┴──┴──┴──┴──┤
   │                   offset[20|10:1|11|19:12]                |      rd      |      opcode        |
   ╰───────────────────────────────────────────────────────────┴──────────────┴────────────────────╯  

    TIPO ECALL

   │31 30 29 28 27 26 25│24 23 22 21 20│19 18 17 16 15│14 13 12│11 10 9  8  7 │6  5  4  3  2  1  0 │
   ├──┴──┴──┴──┴──┴──┴──┼──┴──┴──┴──┴──┼──┴──┴──┴──┴──┼──┴──┴──┼──┴──┴──┴──┴──┼──┴──┴──┴──┴──┴──┴──┤
   │       0            │   func5      |      0       |  0     |     0        |      opcode        |
   ╰────────────────────┴──────────────┴──────────────┴────────┴──────────────┴────────────────────╯           
*/

    //-- TODO
    //----- Modo privilegiado
    //-- csrrw
    //-- csrrs
    //-- csrrc
    //-- csrrwi
    //-- csrrsi
    //-- csrrci
    //-- uret
    //-- sret
    //-- mret
    //-- wfi
    //-- fence
    //-- fence.i
    //-- sfence.vma
    //----RV64I
    //-- addiw
    //-- slliw
    //-- srliw
    //-- sraiw
    //-- addw
    //-- subw
    //-- sllw
    //-- srlw
    //-- sraw
    //-- lwu

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
const FUNC3_POS: u8 = 12;  
const RS1_POS: u8 = 15;
const RS2_POS: u8 = 20;  
const IMM12_POS: u8 = 20; 
const FUNC7_POS: u8 = 25;
const OFFSET4_POS: u8 = 8;
const OFFSET5_POS: u8 = 7;
const OFFSET6_POS: u8 = 25;
const OFFSET7_POS: u8 = 25;
const IMM20_POS: u8 = 12; 
const OFFSET8_POS: u8 = 12;
const OFFSET10_POS: u8 = 21;
const OFFSET1_POS: u8 = 20;







//────────────────────────────────────────────────
//  CALCULAR LAS MASCARAS DE ACCESO A LOS CAMPOS
//────────────────────────────────────────────────
const OPCODE_MASK: u32 = FIELD_7B << OPCODE_POS; 
const RD_MASK: u32 = FIELD_5B << RD_POS;  
const FUNC3_MASK: u32 = FIELD_3B << FUNC3_POS;  
const RS1_MASK: u32 = FIELD_5B << RS1_POS;
const RS2_MASK: u32 = FIELD_5B << RS2_POS;
const IMM12_MASK: u32 = FIELD_12B << IMM12_POS;  
const FUNC7_MASK: u32 = FIELD_7B << FUNC7_POS;
const OFFSET7_MASK: u32 = FIELD_7B << OFFSET7_POS;
const OFFSET6_MASK: u32 = FIELD_6B << OFFSET6_POS;
const OFFSET5_MASK: u32 = FIELD_5B << OFFSET5_POS;
const OFFSET4_MASK: u32 = FIELD_4B << OFFSET4_POS;
const IMM20_MASK: u32 = FIELD_20B << IMM20_POS; 
const OFFSET8_MASK: u32 = FIELD_8B << OFFSET8_POS; 
const OFFSET10_MASK: u32 = FIELD_10B << OFFSET10_POS;
const OFFSET1_MASK: u32 = FIELD_1B << OFFSET1_POS;




//────────────────────────────────────────────────
// Entrada: Valor de 12 bits  
// Salida: Valor extendido a 32 bits con signo
//────────────────────────────────────────────────
fn sign_ext(value: i32) -> i32 {
    //-- Obtener el bit de signo
    //-- sign_bit = true --> negativo
    let sign_bit = (value & 0x800) != 0;

    //-- En caso de ser negativo, extender el signo
    if sign_bit {
        value | !0xFFF  //-- Extender el signo a 32 bits
    } else {
        value  //-- No es negativo, devolver el valor original
    }
}

fn sign_ext13(value: i32) -> i32 {
//────────────────────────────────────────────────
// Entrada: Valor de 13 bits  
// Salida: Valor extendido a 32 bits con signo
//────────────────────────────────────────────────
    //-- Obtener el bit de signo
    //-- sign_bit = true --> negativo
    let sign_bit = (value & 0x1000) != 0;

    //-- En caso de ser negativo, extender el signo
    if sign_bit {
        value | !0x1FFF  //-- Extender el signo a 32 bits
    } else {
        value  //-- No es negativo, devolver el valor original
    }
}

fn sign_ext20(value: i32) -> i32 {
//────────────────────────────────────────────────
// Entrada: Valor de 20 bits  
// Salida: Valor extendido a 32 bits con signo
//────────────────────────────────────────────────
    //-- Obtener el bit de signo
    //-- sign_bit = true --> negativo
    let sign_bit = (value & 0x80000) != 0;

    //-- En caso de ser negativo, extender el signo
    if sign_bit {
        value | !0xFFFFF  //-- Extender el signo a 32 bits
    } else {
        value  //-- No es negativo, devolver el valor original
    }
}

fn sign_ext21(value: i32) -> i32 {
//────────────────────────────────────────────────
// Entrada: Valor de 21 bits  
// Salida: Valor extendido a 32 bits con signo
//────────────────────────────────────────────────
    //-- Obtener el bit de signo
    //-- sign_bit = true --> negativo
    let sign_bit = (value & 0x100000) != 0;

    //-- En caso de ser negativo, extender el signo
    if sign_bit {
        value | !0x1FFFFF  //-- Extender el signo a 32 bits
    } else {
        value  //-- No es negativo, devolver el valor original
    }
}


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

    //────────────────────────────────────────────────
    // Obtener el registro fuente 2 (rs2) de la instrucción
    //────────────────────────────────────────────────
    pub fn rs2(&self) -> Reg {
        let reg_num: u32 = (self.value & RS2_MASK) >> RS2_POS;
        Reg::new(reg_num as u8) 
    }

    //────────────────────────────────────────────────
    //  Obtener el campo func3 de la instrucción
    //────────────────────────────────────────────────    
    pub fn func3(&self) -> u32 {
        (self.value & FUNC3_MASK) >> FUNC3_POS
    }

    //────────────────────────────────────────────────
    //  Obtener el campo func7 de la instrucción
    //────────────────────────────────────────────────    
    pub fn func7(&self) -> u32 {
        (self.value & FUNC7_MASK) >> FUNC7_POS
    }

    //────────────────────────────────────────────────
    //  Obtener el campo inmediato de 12 bits (imm12)
    //────────────────────────────────────────────────
    pub fn imm12(&self) -> i32 {
        //-- Aplicar la máscara para extraer el campo
        //-- y desplazarlo a la posición 0
        let imm12: u32 = (self.value & IMM12_MASK) >> IMM12_POS;

        //-- Convertir el valor a i32 para manejar el signo
        //-- y devolverlo!
        sign_ext(imm12 as i32)
    }

    //────────────────────────────────────────────────
    //  Obtener el campo offset de las instrucciones S
    //────────────────────────────────────────────────
    pub fn offset_s(&self) -> i32 {
        //-- Extraer el campo offset7
        let offset7: u32 = (self.value & OFFSET7_MASK) >> OFFSET7_POS;
        let offset5: u32 = (self.value & OFFSET5_MASK) >> OFFSET5_POS;
        let offset: u32 = offset7 << 5 | offset5;

        sign_ext(offset as i32)
    }

    //────────────────────────────────────────────────
    //  Obtener el campo offset de las instrucciones B
    //────────────────────────────────────────────────
    pub fn offset_b(&self) -> i32 {

        //-- Extraer el bit de signo
        let sign: u32 = (self.value & 0x8000_0000) >> 31;

        //-- Extraer el bit 11, de la posicion 7
        let b11: u32 = (self.value & 0x0000_0080) >> 7;

        let offset6: u32 = (self.value & OFFSET6_MASK) >> OFFSET6_POS;

        let offset4: u32 = (self.value & OFFSET4_MASK) >> OFFSET4_POS;

        //-- Construir el offset final a partir de todos sus componentes
        let offset: u32 = (sign << 12) | (b11 << 11) |  
                                  (offset6 << 5) | offset4<<1;    
        sign_ext13(offset as i32)
    }

    pub fn imm20(&self) -> i32 {
    //────────────────────────────────────────────────
    // Entrada: Instrucción RISC-V
    // Salida: Inmediato de 20 bits de la instrucción
    //────────────────────────────────────────────────
          //-- Aplicar la máscara para extraer el campo
          //-- y desplazarlo a la posición 0
          let imm20: u32 = (self.value & IMM20_MASK) >> IMM20_POS;

          //-- Convertir el valor a i32
          //-- Hay que extender el signo
          sign_ext20(imm20 as i32)
    }

    pub fn offset_jal(&self) -> i32 {
    //────────────────────────────────────────────────
    // Entrada: Instrucción RISC-V
    // Salida: Valor del offset para instrucciones jal
    // offset[20|10:1|11|19:12]
    //────────────────────────────────────────────────
        //-- Extraer el bit de signo
        let sign: u32 = (self.value & 0x8000_0000) >> 31;
        
        //-- Extraer los bits 19-12
        let offset8: u32 = (self.value & OFFSET8_MASK) >> OFFSET8_POS;
        
        //-- Extraer el bit 11
        let b11: u32 = (self.value & OFFSET1_MASK) >> OFFSET1_POS;
        
        //-- Extraer los bits 10-1
        let offset10: u32 = (self.value & OFFSET10_MASK) >> OFFSET10_POS;
        
        //-- Construir el offset final a partir de todos sus componentes
        let offset: u32 = (sign << 20) | (offset8 << 12) | b11 << 11 |
                          (offset10 << 1); 
                      
        sign_ext21(offset as i32)
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

#[test]
fn test_rs2() {
    //--                               func7  rs2   rs1  func3 rd   opcode
    let mcode = MCode::new(0b_0000000_00000_00000_000_00000_0000000); 
    assert_eq!(mcode.rs2().to_str(), "x0");

    let mcode = MCode::new(0b_0000000_00001_00000_000_00000_0000000); 
    assert_eq!(mcode.rs2().to_str(), "x1");

    let mcode = MCode::new(0b_0000000_00010_00000_000_00000_0000000); 
    assert_eq!(mcode.rs2().to_str(), "x2");

    let mcode = MCode::new(0b_0000000_00100_00000_000_00000_0000000); 
    assert_eq!(mcode.rs2().to_str(), "x4");

    let mcode = MCode::new(0b_0000000_01000_00000_000_00000_0000000); 
    assert_eq!(mcode.rs2().to_str(), "x8");

    let mcode = MCode::new(0b_0000000_10000_00000_000_00000_0000000); 
    assert_eq!(mcode.rs2().to_str(), "x16");

    let mcode = MCode::new(0b_0000000_10001_00000_000_00000_0000000); 
    assert_eq!(mcode.rs2().to_str(), "x17");

    let mcode = MCode::new(0b_0000000_11111_00000_000_00000_0000000); 
    assert_eq!(mcode.rs2().to_str(), "x31");
}
    
#[test]
fn test_func3() {
    assert_eq!(MCode::new(0b_0000000_00000_00000_000_00000_0000000).func3(), 0b000);
    assert_eq!(MCode::new(0b_0000000_00000_00000_001_00000_0000000).func3(), 0b001);
    assert_eq!(MCode::new(0b_0000000_00000_00000_010_00000_0000000).func3(), 0b010);
    assert_eq!(MCode::new(0b_0000000_00000_00000_100_00000_0000000).func3(), 0b100);
    assert_eq!(MCode::new(0b_0000000_00000_00000_111_00000_0000000).func3(), 0b111);
}

#[test]
fn test_func7() {
  //--                      func7  rs2   rs1  func3 rd    opcode
  assert_eq!(MCode::new(0b_0000000_00000_00000_000_00000_0000000).func7(), 0b0000000);
  assert_eq!(MCode::new(0b_0000001_00000_00000_000_00000_0000000).func7(), 0b0000001);
  assert_eq!(MCode::new(0b_0000010_00000_00000_000_00000_0000000).func7(), 0b0000010);
  assert_eq!(MCode::new(0b_0000100_00000_00000_000_00000_0000000).func7(), 0b0000100);
  assert_eq!(MCode::new(0b_0001000_00000_00000_000_00000_0000000).func7(), 0b0001000);
  assert_eq!(MCode::new(0b_0010000_00000_00000_000_00000_0000000).func7(), 0b0010000);
  assert_eq!(MCode::new(0b_0100000_00000_00000_000_00000_0000000).func7(), 0b0100000);
  assert_eq!(MCode::new(0b_1000000_00000_00000_000_00000_0000000).func7(), 0b1000000);
  assert_eq!(MCode::new(0b_1111111_00000_00000_000_00000_0000000).func7(), 0b1111111);
}

#[test]
fn test_imm12() {

    assert_eq!(
        MCode::new(0b_0000_0000_0000__00000__000__00000__0000000).imm12(), 
        0b0000_0000_0000);
    assert_eq!(
        MCode::new(0b_0000_0000_0001__00000__000__00000__0000000).imm12(), 
        0x001);
    assert_eq!(
        MCode::new(0b_0000_0000_0010__00000__000__00000__0000000).imm12(), 
        0x002);
    assert_eq!(
        MCode::new(0b_0000_0000_0100__00000__000__00000__0000000).imm12(), 
        0x004);
    assert_eq!(
        MCode::new(0b_0000_0000_1000__00000__000__00000__0000000).imm12(), 
        0x008);
    assert_eq!(
        MCode::new(0b_0000_0001_0000__00000__000__00000__0000000).imm12(), 
        0x010);
    assert_eq!(
        MCode::new(0b_0000_0010_0000__00000__000__00000__0000000).imm12(), 
        0x020);
    assert_eq!(
        MCode::new(0b_0000_0100_0000__00000__000__00000__0000000).imm12(), 
        0x040);
    assert_eq!(
        MCode::new(0b_0000_1000_0000__00000__000__00000__0000000).imm12(), 
        0x080);
    assert_eq!(
        MCode::new(0b_0001_0000_0000__00000__000__00000__0000000).imm12(), 
        0x100);
    assert_eq!(
        MCode::new(0b_0010_0000_0000__00000__000__00000__0000000).imm12(), 
        0x200);
    assert_eq!(
        MCode::new(0b_0100_0000_0000__00000__000__00000__0000000).imm12(), 
        0x400);

  //-- Pruebs de signo
    assert_eq!(MCode::new(0x800_0_0000).imm12(), 0xFFFF_F800u32 as i32);
    assert_eq!(MCode::new(0xFFF_0_0000).imm12(), 0xFFFF_FFFFu32 as i32);
    assert_eq!(MCode::new(0xFFF_FFFFF).imm12(), -1);
    assert_eq!(MCode::new(0x800_FFFFF).imm12(), -2048);
    assert_eq!(MCode::new(0x7FF_FFFFF).imm12(), 2047);
    assert_eq!(MCode::new(0xFFE_FFFFF).imm12() as i32, -2);
}


//────────────────────────────────────────────────
//  TESTS
//────────────────────────────────────────────────
#[test]
fn test_sign_ext() {
  //-- Test de la función sign_ext
  assert_eq!(sign_ext(0x000), 0);
  assert_eq!(sign_ext(0x001), 1);
  assert_eq!(sign_ext(0x7FF), 2047);  //-- 0x7FF
  assert_eq!(sign_ext(0x800), -2048); //-- 0x800
  assert_eq!(sign_ext(0xFFF), -1);
  assert_eq!(sign_ext(0x7FF_FFFF), -1);
}
