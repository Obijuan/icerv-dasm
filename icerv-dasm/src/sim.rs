//────────────────────────────────────────────────
//  FUNCIONES DE SIMULACION DE RV32I
//────────────────────────────────────────────────
use crate::ansi;
use crate::memory::Memory;
use crate::cpurv::Cpurv;

//────────────────────────────────────────────────
//  Simular un programa de test y comprobar
//  si lo ha pasado correctamente
//────────────────────────────────────────────────
pub fn sim_test(fich: &str, max_cycles: u32)
{

    //-- Simular el programa
    let cpu = sim(fich, max_cycles);

    //-- Comprobar si se ha pasado el test o no
    //-- Esto lo sabemos examinando el valor del registro x1
    //-- que debe ser 1 si el test ha pasado correctamente
    assert_eq!(cpu.x1, 1);
    cpu.show();
    println!("✅ TEST PASSED!!");
}

//────────────────────────────────────────────────
//  Simular un programa almacenado en un fichero
//  Se devuelve la cpu creada para analizar su estado
//  tras la simulacion
//──────────────────────────────────────────────── 
pub fn sim(fich: &str, max_cycles: u32) -> Cpurv
{
    //-- Crear memoria e inicializarla desde un fichero
    let mem = Memory::from_file(fich);

    println!();
    println!("{}{}{}",ansi::BLUE, fich, ansi::RESET);
    println!("Tamaño: {} Instrucciones", mem.size()>>2);

    //-- Crear CPU
    let mut cpu = Cpurv::new(mem);

    //-- Mostrar estado inicial de la cpu
    cpu.show();

    cpu.run(max_cycles);

    println!("PROGRAMA TERMINADO");

    cpu
}

