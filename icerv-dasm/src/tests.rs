#[test]
fn test_disassemble_ecall_ebreak() {
    assert_eq!(disassemble(0x00000073), "ecall");
    assert_eq!(disassemble(0x00100073), "ebreak");
}

