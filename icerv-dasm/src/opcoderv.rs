//────────────────────────────────────────────────
//  DEFINICION DE LOS OPCODES
//────────────────────────────────────────────────
pub enum OpcodeRV {
  //-- Instrucciones aritméticas tipo I
  TipoIArith = 0b_00100_11,   //-- Ex: ADDI: addi rd, rs1, imm12

  //-- Instrucciones de carga (tipo I)
  TipoILoad = 0b_00000_11,    //-- Ex: LW: lw rd, imm12(rs1)

  //-- Instrucciones tipo-R
  TipoR = 0b_01100_11,       //-- Ex: ADD: add rd, rs1, rs2

  //-- Instrucciones tipo-S
  TipoS = 0b_01000_11,        //-- Ex: SW: sw rs2, imm12(rs1)

  //-- Instrucciones tipo-B
  TipoB = 0b_11000_11,        //-- Ex: BEQ: beq rs1, rs2, offset

  //-- Instruccion tipo-U: LUI
  TipoULui = 0b_01101_11,   //-- Ex: LUI: lui rd, imm20

  //-- Instruccion tipo-U: AUIPC
  TipoUAuipc = 0b_00101_11, //-- Ex: AUIPC: auipc rd, imm20

  //-- Instruccion tipo-J: jal
  TipoJJal = 0b_11011_11, //-- Ex: JAL: jal rd, offset

  //-- Instruccion tipo-J: jalr
  TipoJJalr = 0b_11001_11, //-- Ex: JALR: jalr rd, rs1, imm12

  //-- Instruccion ecall/ebreak
  TipoEcallEbreak = 0b_11100_11, //-- Ex: ECALL: ecall

  //-- Instrucción desconocida
  Unknown,
}

