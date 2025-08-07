//────────────────────────────────────────────────
//  PROGRAMA DE PRUEBAS
//  Ejecutable usado para importar los módulos creados
//    en el paquete icerv-dasm
//────────────────────────────────────────────────

use icerv_dasm::regs::Reg;
use icerv_dasm::opcoderv::OpcodeRV;
use icerv_dasm::mcode::MCode;
use icerv_dasm::instructionrv::InstructionRV;

fn main() {
    println!("Programa test1.rs...");

    println!("--> Modulo reg:");
    assert_eq!(Reg::X0.to_str(), "x0");
    println!("  Registro {}", Reg::X0.to_str());

    println!("--> Módulo OpcodeRV: ");
    println!("  Opcode {}", OpcodeRV::TipoB as u8);

    println!("--> Modulo Mcode:");
    let mcode = MCode::new(0x00000013);
    assert_eq!(mcode.opcode() as u32, 0x13);
    println!("  Opcode: {:#X}", mcode.opcode() as u8);

    println!("--> Módulo InstructionRV:");
    //-- Leer instrucción a partir del código máquina
    let inst= InstructionRV::from_mcode(0x00100093);
    println!("  Instruccion: {}", inst.to_string());
}

