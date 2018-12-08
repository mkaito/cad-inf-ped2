use std::io::{self, Write, Read};

pub fn leer_numero(mensaje: &str) -> u32 {
    let mut buffer = String::new();
    print!("{}: ", mensaje);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer)
        .expect("Ha ocurrido un error al leer stdin.");

    buffer.trim().parse::<u32>()
        .expect("El dato introducido no es un número válido")
}

pub fn pause(mensaje: &str) {
    print!("{}", mensaje);
    io::stdout().flush().unwrap();
    let _ = io::stdin().read(&mut [0]).unwrap();
}
