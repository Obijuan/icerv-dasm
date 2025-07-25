use crate::{mcode, regs::Reg, opcoderv::OpcodeRV};


pub enum InstructionRV {
//────────────────────────────────────────────────
//  Instrucciones del RISC-V
//────────────────────────────────────────────────    
  Addi {rd: Reg, rs1: Reg, imm: i32},  //-- addi rd, rs1, imm12
  Slli {rd: Reg, rs1: Reg, imm: i32},  //-- slli x1, x2, 1
  Unknown, //-- Instrucción desconocida
}

impl InstructionRV {
    pub fn from_mcode(mcode: u32) -> Self {

        let mcode = mcode::MCode::new(mcode);

        match mcode.opcode() {
            OpcodeRV::TipoIArith => {
                Self::Addi {
                    rd: mcode.rd(),
                    rs1: mcode.rs1(),
                    imm: mcode.imm12()
                }

            },
            _ => Self::Unknown,
        }    
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Addi {rd, rs1, imm} => {
                format!("addi {}, {}, {}", rd.to_str(), rs1.to_str(), imm)
            },
            Self::Slli {rd, rs1, imm} => {
                format!("slli {}, {}, {}", rd.to_str(), rs1.to_str(), imm)
            },
            Self::Unknown => {
                "Unknown Instruction".to_string()
            },
        }
    }
}

#[test]
fn test_instructions_addi() {

    assert_eq!(
      InstructionRV::Addi { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_string(),
      "addi x0, x0, 0");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X1, rs1: Reg::X0, imm: 1 }.to_string(),
      "addi x1, x0, 1");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X2, rs1: Reg::X0, imm: 2 }.to_string(),
      "addi x2, x0, 2");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X3, rs1: Reg::X0, imm: -1 }.to_string(),
      "addi x3, x0, -1");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X4, rs1: Reg::X0, imm: 2047 }.to_string(),
      "addi x4, x0, 2047");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X31, rs1: Reg::X1, imm: 3 }.to_string(),
      "addi x31, x1, 3");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X8, rs1: Reg::X2, imm: 4 }.to_string(),
      "addi x8, x2, 4");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X16, rs1: Reg::X4, imm: 8 }.to_string(),
      "addi x16, x4, 8");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X17, rs1: Reg::X8, imm: 16 }.to_string(),
      "addi x17, x8, 16");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X17, rs1: Reg::X8, imm: -16 }.to_string(),
      "addi x17, x8, -16");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X17, rs1: Reg::X8, imm: -2048 }.to_string(),
      "addi x17, x8, -2048");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X1, rs1: Reg::X0, imm: 170 }.to_string(),
      "addi x1, x0, 170");
}


#[test]
fn test_mcode_addi() {
    //────────────────────────────────────────────────
    //  Test de la instrucción ADDI
    //────────────────────────────────────────────────
    assert_eq!(
        InstructionRV::from_mcode(0x00000013).to_string(),
        "addi x0, x0, 0");
    assert_eq!(
        InstructionRV::from_mcode(0x00100093).to_string(),
        "addi x1, x0, 1");
    assert_eq!(
        InstructionRV::from_mcode(0x00200113).to_string(),
        "addi x2, x0, 2");
    assert_eq!(
        InstructionRV::from_mcode(0xfff00193).to_string(),
        "addi x3, x0, -1");
    assert_eq!(
        InstructionRV::from_mcode(0x7ff00213).to_string(),
        "addi x4, x0, 2047");
    assert_eq!(
        InstructionRV::from_mcode(0x00308f93).to_string(),
        "addi x31, x1, 3"); 
    assert_eq!(
        InstructionRV::from_mcode(0x00410413).to_string(),
        "addi x8, x2, 4");
    assert_eq!(
        InstructionRV::from_mcode(0x00820813).to_string(),
        "addi x16, x4, 8");
    assert_eq!(
        InstructionRV::from_mcode(0x01040893).to_string(),
        "addi x17, x8, 16");
    assert_eq!(
        InstructionRV::from_mcode(0xff040893).to_string(),
        "addi x17, x8, -16");
    assert_eq!(
        InstructionRV::from_mcode(0x80040893).to_string(),
        "addi x17, x8, -2048");
    assert_eq!(
        InstructionRV::from_mcode(0x0aa00093).to_string(),
        "addi x1, x0, 170");  
}   

#[test]
fn test_instructions_slli() {

    assert_eq!(
      InstructionRV::Slli { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_string(),
      "slli x0, x0, 0");
    assert_eq!(
      InstructionRV::Slli { rd: Reg::X1, rs1: Reg::X2, imm: 1 }.to_string(),
      "slli x1, x2, 1");
    assert_eq!(
      InstructionRV::Slli { rd: Reg::X31, rs1: Reg::X1, imm: 2 }.to_string(),
      "slli x31, x1, 2");
    assert_eq!(
      InstructionRV::Slli { rd: Reg::X30, rs1: Reg::X2, imm: 4 }.to_string(),
      "slli x30, x2, 4");
    assert_eq!(
      InstructionRV::Slli { rd: Reg::X29, rs1: Reg::X3, imm: 8 }.to_string(),
      "slli x29, x3, 8");
    assert_eq!(
      InstructionRV::Slli { rd: Reg::X28, rs1: Reg::X4, imm: 16 }.to_string(),
      "slli x28, x4, 16");
    assert_eq!(
      InstructionRV::Slli { rd: Reg::X27, rs1: Reg::X5, imm: 17 }.to_string(),
      "slli x27, x5, 17");
    assert_eq!(
      InstructionRV::Slli { rd: Reg::X26, rs1: Reg::X6, imm: 30 }.to_string(),
      "slli x26, x6, 30");
    assert_eq!(
      InstructionRV::Slli { rd: Reg::X25, rs1: Reg::X7, imm: 31 }.to_string(),
      "slli x25, x7, 31");

}

