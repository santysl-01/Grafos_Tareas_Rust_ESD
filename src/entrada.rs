use std::io::{self, Write};

pub fn pausa(mensaje: &str) {
    print!("\n{} (enter para continuar) ", mensaje);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

pub fn leer_linea(mensaje: &str) -> String {
    print!("{}", mensaje);
    io::stdout().flush().unwrap();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    entrada.trim().to_string()
}

pub fn leer_numero(mensaje: &str) -> usize {
    loop {
        let texto = leer_linea(mensaje);
        match texto.parse::<usize>() {
            Ok(n) => return n,
            Err(_) => println!("eso no es un numero, intenta otra vez"),
        }
    }
}