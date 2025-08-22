//────────────────────────────────────────────────
//  SIMULADOR DE RV32I
//────────────────────────────────────────────────
use icerv_dasm::sim::sim;
use icerv_dasm::ansi;


fn main() 
{

    //-- Borrar la pantalla
    print!("{}", ansi::CLS);

    //-- Leer primer argumento
    let arg = std::env::args().nth(1);
    let fich = match arg {
        Some(value) => {
            value
        }
        None => {
            print!("{}", ansi::RED);
            println!("Error: Fichero ejecutable NO especificado");
            print!("{}", ansi::RESET);
            println!("  Uso: sim fichero");
            return;
        }
    };

    //-- Ejecutar programa
    sim(&fich, 10);
}


