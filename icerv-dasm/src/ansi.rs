//────────────────────────────────────────────────
//  MODULO ANSI
//────────────────────────────────────────────────

//────────────────────────────────────────
//  Constantes con los códigos de escape
//────────────────────────────────────────

//── Borrar la pantalla y poner el cursor en home
pub const CLS: &str = "\x1b[H\x1b[2J";

//── Resetear los atributos
pub const RESET: &str = "\x1B[0m";

//── Colores
pub const GREEN: &str = "\x1b[0;32m";
pub const YELLOW: &str = "\x1b[0;33m";
pub const BLUE: &str = "\x1b[0;34m";
