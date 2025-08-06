use crate::disassemble;

#[test]
fn test_disassemble_ecall_ebreak() {
    assert_eq!(disassemble(0x00000073), "ecall");
    assert_eq!(disassemble(0x00100073), "ebreak");
}

#[test]
fn test_disassemble_jal() {
    assert_eq!(disassemble(0x0000006f), "jal x0, 0");
    assert_eq!(disassemble(0xffdff0ef), "jal x1, -4");
    assert_eq!(disassemble(0xff9ff16f), "jal x2, -8");
    assert_eq!(disassemble(0xff5ff1ef), "jal x3, -12");
    assert_eq!(disassemble(0xff1ff26f), "jal x4, -16");
    assert_eq!(disassemble(0x014002ef), "jal x5, 20");
    assert_eq!(disassemble(0x0100036f), "jal x6, 16");
    assert_eq!(disassemble(0x00c003ef), "jal x7, 12");
    assert_eq!(disassemble(0x0080046f), "jal x8, 8");
    assert_eq!(disassemble(0x004004ef), "jal x9, 4");
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

#[test]
fn test_disassemble_auipc() {
    assert_eq!(disassemble(0x00000017), "auipc x0, 0x00000");
    assert_eq!(disassemble(0x00001097), "auipc x1, 0x00001");
    assert_eq!(disassemble(0x00020117), "auipc x2, 0x00020");
    assert_eq!(disassemble(0x00400197), "auipc x3, 0x00400");
    assert_eq!(disassemble(0x08000217), "auipc x4, 0x08000");
    assert_eq!(disassemble(0x7ffff297), "auipc x5, 0x7FFFF");
    assert_eq!(disassemble(0x80000317), "auipc x6, 0x80000");
    assert_eq!(disassemble(0xfffff397), "auipc x7, 0xFFFFF");
}








