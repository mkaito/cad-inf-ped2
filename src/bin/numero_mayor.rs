use std::io::{self, Write};

fn main() {
    println!("Hola!");

    let mut buffer = String::new();
    print!("Introduzca el primer número: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer)
        .expect("Ha ocurrido un error al leer stdin.");
    let n1: u32 = buffer.trim().parse().unwrap();

    buffer = String::new();
    print!("Introduzca el segundo número: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer)
        .expect("Ha ocurrido un error al leer stdin.");
    let n2: u32 = buffer.trim().parse().unwrap();

    if n1 == n2 { println!("Son iguales!"); }

    let nm = if n1 > n2 { n1 } else { n2 };
    println!("El número mayor es {}.", nm);
    println!("Gracias y hasta luego.")
}
