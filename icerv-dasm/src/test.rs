use icerv_dasm::instructionrv::InstructionRV;
use icerv_dasm::regs::Reg;

#[test]
fn test1() {
    //-- Instruciones Tipo I
  assert_eq!(
    InstructionRV::Addi{rd: Reg::X1, rs1: Reg::X0, imm: 1}.to_string(),
    "addi x1, x0, 1");
  assert_eq!(
    InstructionRV::Slli{rd: Reg::X1, rs1: Reg::X2, imm: 1}.to_string(),
    "slli x1, x2, 1");
  assert_eq!(
    InstructionRV::Slti{rd: Reg::X1, rs1: Reg::X2, imm: 1}.to_string(),
    "slti x1, x2, 1");
  assert_eq!(
    InstructionRV::Sltiu{rd: Reg::X1, rs1: Reg::X2, imm: 1}.to_string(),
    "sltiu x1, x2, 1");
  assert_eq!(
    InstructionRV::Xori{rd: Reg::X1, rs1: Reg::X2, imm: 1}.to_string(),
    "xori x1, x2, 1");
  assert_eq!(
    InstructionRV::Srli{rd: Reg::X1, rs1: Reg::X2, imm: 1}.to_string(),
    "srli x1, x2, 1");
  assert_eq!(
    InstructionRV::Ori{rd: Reg::X1, rs1: Reg::X2, imm: 1}.to_string(),
    "ori x1, x2, 1");
  assert_eq!(
    InstructionRV::Andi{rd: Reg::X1, rs1: Reg::X2, imm: 1}.to_string(),
    "andi x1, x2, 1");
  assert_eq!(
    InstructionRV::Srai{rd: Reg::X1, rs1: Reg::X2, imm: 1}.to_string(),
    "srai x1, x2, 1");
  assert_eq!(
    InstructionRV::Lb { rd: Reg::X0, offs: 0, rs1: Reg::X1 }.to_string(),
    "lb x0, 0(x1)");
  assert_eq!(
    InstructionRV::Lh { rd: Reg::X0, offs: 0, rs1: Reg::X1 }.to_string(),
    "lh x0, 0(x1)");
  assert_eq!(
    InstructionRV::Lw { rd: Reg::X0, offs: 0, rs1: Reg::X1 }.to_string(),
    "lw x0, 0(x1)");
  assert_eq!(
    InstructionRV::Ld { rd: Reg::X0, offs: 0, rs1: Reg::X1 }.to_string(),
    "ld x0, 0(x1)");
  assert_eq!(
    InstructionRV::Lbu { rd: Reg::X0, offs: 0, rs1: Reg::X1 }.to_string(),
    "lbu x0, 0(x1)");
  assert_eq!(
    InstructionRV::Lhu { rd: Reg::X0, offs: 0, rs1: Reg::X1 }.to_string(),
    "lhu x0, 0(x1)");
  assert_eq!(
    InstructionRV::Lwu { rd: Reg::X0, offs: 0, rs1: Reg::X1 }.to_string(),
    "lwu x0, 0(x1)");

  //-- Instruciones Tipo R
  assert_eq!(
    InstructionRV::Add { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(),
    "add x0, x1, x2");
  assert_eq!(
    InstructionRV::Sub { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(),
    "sub x0, x1, x2");
  assert_eq!(
    InstructionRV::Sll { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(),
    "sll x0, x1, x2");
  assert_eq!(
    InstructionRV::Sltu { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(),
    "sltu x0, x1, x2");
  assert_eq!(
    InstructionRV::Xor { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(),
    "xor x0, x1, x2");
  assert_eq!(
    InstructionRV::Srl { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(),
    "srl x0, x1, x2");
  assert_eq!(
    InstructionRV::Or { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(),
    "or x0, x1, x2");
  assert_eq!(
    InstructionRV::Sra { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(),
    "sra x0, x1, x2");

  //-- Instruciones Tipo S
  assert_eq!(
    InstructionRV::Sb { rs2: Reg::X0, offs: 0, rs1: Reg::X1}.to_string(),
    "sb x0, 0(x1)");
  assert_eq!(
    InstructionRV::Sh { rs2: Reg::X2, offs: -1, rs1: Reg::X3}.to_string(),
    "sh x2, -1(x3)");
  assert_eq!(
    InstructionRV::Sw { rs2: Reg::X4, offs: 2047, rs1: Reg::X5}.to_string(),
    "sw x4, 2047(x5)");
  assert_eq!(
    InstructionRV::Sd { rs2: Reg::X5, offs: -2048, rs1: Reg::X6}.to_string(),
    "sd x5, -2048(x6)");

  //-- Instructiones Tipo B
  assert_eq!(
    InstructionRV::Beq {rs1: Reg::X1, rs2: Reg::X2, offs: -4}.to_string(),
    "beq x1, x2, -4");
  assert_eq!(
    InstructionRV::Bne {rs1: Reg::X3, rs2: Reg::X4, offs: -8}.to_string(),
    "bne x3, x4, -8");
  assert_eq!(
    InstructionRV::Blt {rs1: Reg::X5, rs2: Reg::X6, offs: -12}.to_string(),
    "blt x5, x6, -12");
  assert_eq!(
    InstructionRV::Bge {rs1: Reg::X7, rs2: Reg::X8, offs: 24}.to_string(),
    "bge x7, x8, 24");
  assert_eq!(
    InstructionRV::Bltu {rs1: Reg::X9, rs2: Reg::X10, offs: 20}.to_string(),
    "bltu x9, x10, 20");
  assert_eq!(
    InstructionRV::Bgeu {rs1: Reg::X11, rs2: Reg::X12, offs: 16}.to_string(),
    "bgeu x11, x12, 16");

  //-- Instrucciones Tipo U
  assert_eq!(
    InstructionRV::Lui {rd: Reg::X6, imm: 0x80000}.to_string(),
    "lui x6, 0x80000");
  assert_eq!(
    InstructionRV::Auipc {rd: Reg::X4, imm: 0x08000}.to_string(),
    "auipc x4, 0x08000");

  //-- Instrucciones tipo J
  assert_eq!(
    InstructionRV::Jal {rd: Reg::X4, offs: -16}.to_string(),
    "jal x4, -16");
  assert_eq!(
    InstructionRV::Jalr {rd: Reg::X1, offs: -1, rs1: Reg::X10}.to_string(),
    "jalr x1, -1(x10)");

  //-- Instrucciones tipo ecall
  assert_eq!(InstructionRV::Ecall.to_string(), "ecall");
  assert_eq!(InstructionRV::Ebreak.to_string(), "ebreak");
}


#[test]
fn test2() {
    assert_eq!(InstructionRV::from_mcode(0x00100093).to_string(), 
               "addi x1, x0, 1");
    assert_eq!(InstructionRV::from_mcode(0x00111093).to_string(), 
               "slli x1, x2, 1");
    assert_eq!(InstructionRV::from_mcode(0x00112093).to_string(), 
               "slti x1, x2, 1");
    assert_eq!(InstructionRV::from_mcode(0x00113093).to_string(), 
               "sltiu x1, x2, 1");
    assert_eq!(InstructionRV::from_mcode(0x00114093).to_string(), 
               "xori x1, x2, 1");
    assert_eq!(InstructionRV::from_mcode(0x00115093).to_string(), 
               "srli x1, x2, 1");
    assert_eq!(InstructionRV::from_mcode(0x00116093).to_string(), 
               "ori x1, x2, 1");    
    assert_eq!(InstructionRV::from_mcode(0x00117093).to_string(), 
               "andi x1, x2, 1");
    assert_eq!(InstructionRV::from_mcode(0x40115093).to_string(), 
               "srai x1, x2, 1");
    assert_eq!(InstructionRV::from_mcode(0x00008003).to_string(), 
               "lb x0, 0(x1)");   
    assert_eq!(InstructionRV::from_mcode(0x00009003).to_string(), 
               "lh x0, 0(x1)");   
    assert_eq!(InstructionRV::from_mcode(0x0000a003).to_string(), 
               "lw x0, 0(x1)");   
    assert_eq!(InstructionRV::from_mcode(0x0000b003).to_string(), 
               "ld x0, 0(x1)");   
    assert_eq!(InstructionRV::from_mcode(0x0000c003).to_string(), 
               "lbu x0, 0(x1)");    
    assert_eq!(InstructionRV::from_mcode(0x0000d003).to_string(), 
               "lhu x0, 0(x1)");    
    assert_eq!(InstructionRV::from_mcode(0x0000e003).to_string(), 
               "lwu x0, 0(x1)");    
    assert_eq!(InstructionRV::from_mcode(0x00208033).to_string(), 
               "add x0, x1, x2");
    assert_eq!(InstructionRV::from_mcode(0x40208033).to_string(), 
               "sub x0, x1, x2");
    assert_eq!(InstructionRV::from_mcode(0x00209033).to_string(), 
               "sll x0, x1, x2");
    assert_eq!(InstructionRV::from_mcode(0x0020a033).to_string(), 
               "slt x0, x1, x2");
    assert_eq!(InstructionRV::from_mcode(0x0020b033).to_string(), 
               "sltu x0, x1, x2");
    assert_eq!(InstructionRV::from_mcode(0x0020c033).to_string(), 
               "xor x0, x1, x2");
    assert_eq!(InstructionRV::from_mcode(0x0020d033).to_string(), 
               "srl x0, x1, x2");
    assert_eq!(InstructionRV::from_mcode(0x0020e033).to_string(), 
               "or x0, x1, x2");    
    assert_eq!(InstructionRV::from_mcode(0x4020d033).to_string(), 
               "sra x0, x1, x2");
    assert_eq!(InstructionRV::from_mcode(0x00008023).to_string(), 
               "sb x0, 0(x1)");   
    assert_eq!(InstructionRV::from_mcode(0xfe219fa3).to_string(), 
               "sh x2, -1(x3)");    
    assert_eq!(InstructionRV::from_mcode(0x7e42afa3).to_string(), 
               "sw x4, 2047(x5)");
    assert_eq!(InstructionRV::from_mcode(0x80533023).to_string(), 
               "sd x5, -2048(x6)");
    assert_eq!(InstructionRV::from_mcode(0xfe208ee3).to_string(), 
               "beq x1, x2, -4");
    assert_eq!(InstructionRV::from_mcode(0xfe419ce3).to_string(), 
               "bne x3, x4, -8");
    assert_eq!(InstructionRV::from_mcode(0xfe62cae3).to_string(), 
               "blt x5, x6, -12");
    assert_eq!(InstructionRV::from_mcode(0x0083dc63).to_string(), 
               "bge x7, x8, 24");
    assert_eq!(InstructionRV::from_mcode(0x00a4ea63).to_string(), 
               "bltu x9, x10, 20");
    assert_eq!(InstructionRV::from_mcode(0x00c5f863).to_string(), 
               "bgeu x11, x12, 16");
    assert_eq!(InstructionRV::from_mcode(0x80000337).to_string(), 
               "lui x6, 0x80000");
    assert_eq!(InstructionRV::from_mcode(0x08000217).to_string(), 
               "auipc x4, 0x08000");
    assert_eq!(InstructionRV::from_mcode(0xff1ff26f).to_string(), 
               "jal x4, -16");  
    assert_eq!(InstructionRV::from_mcode(0xfff500e7).to_string(), 
               "jalr x1, -1(x10)");
    assert_eq!(InstructionRV::from_mcode(0x00000073).to_string(), 
               "ecall");
    assert_eq!(InstructionRV::from_mcode(0x00100073).to_string(), 
               "ebreak");
}
