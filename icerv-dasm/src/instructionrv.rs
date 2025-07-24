use crate::{mcode, regs::Reg};

pub enum InstructionRV {
//────────────────────────────────────────────────
//  Instrucciones del RISC-V
//────────────────────────────────────────────────    
  Addi {rd: Reg, rs1: Reg, imm: i32},  //-- addi rd, rs1, imm12
}

impl InstructionRV {
    pub fn from_mcode(mcode: u32) -> Self {

        let mcode = mcode::MCode::new(mcode);

        Self::Addi {
          rd: mcode.rd(),
          rs1: mcode.rs1(),
          imm: mcode.imm12(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Addi {rd, rs1, imm} => {
                format!("addi {}, {}, {}", rd.to_str(), rs1.to_str(), imm)
            }
        }
    }
}

#[test]
fn test_instructions() {

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
        InstructionRV::from_mcode(0x00100093).to_string(),
        "addi x1, x0, 1");
}   

