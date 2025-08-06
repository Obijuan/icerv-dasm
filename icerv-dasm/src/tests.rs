use crate::disassemble;

#[test]
fn test_disassemble_ecall_ebreak() {
    assert_eq!(disassemble(0x00000073), "ecall");
    assert_eq!(disassemble(0x00100073), "ebreak");
}

#[test]
fn test_disassemble_jalr() {
    assert_eq!(disassemble(0x00000067), "jalr x0, 0(x0)");
    assert_eq!(disassemble(0xfff500e7), "jalr x1, -1(x10)");
    assert_eq!(disassemble(0xffe58167), "jalr x2, -2(x11)");
    assert_eq!(disassemble(0xffc601e7), "jalr x3, -4(x12)");
    assert_eq!(disassemble(0xff868267), "jalr x4, -8(x13)");
    assert_eq!(disassemble(0x800702e7), "jalr x5, -2048(x14)");
    assert_eq!(disassemble(0x7ff78367), "jalr x6, 2047(x15)");
    assert_eq!(disassemble(0x010803e7), "jalr x7, 16(x16)");
    assert_eq!(disassemble(0x00888467), "jalr x8, 8(x17)");
    assert_eq!(disassemble(0x004904e7), "jalr x9, 4(x18)");
}








