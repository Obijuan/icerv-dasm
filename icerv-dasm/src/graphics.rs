

pub fn lineh(size: usize) 
//────────────────────────────────────────────────
//  Dibujar una linea horizontal (simple)
//    
//  ENTRADAS:
//    -size: Tamaño de la linea en caracteres
//────────────────────────────────────────────────
{
    for _ in 0..size {
        print!("─");
    }
    println!();
}
