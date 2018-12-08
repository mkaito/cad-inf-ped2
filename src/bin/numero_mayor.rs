use std::io::{self, Write};
extern crate dont_disappear;

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

    // Escribir un mensaje dependiendo de la comparación entre los dos números
    if n1 == n2 {
        println!("Son iguales!");
    } else {
        println!("El número mayor es {}.",
                 if n1 > n2 { n1 } else { n2 });
    }

    println!("Gracias y hasta luego.");
    dont_disappear::any_key_to_continue::custom_msg("Presione cualquier tecla para continuar...");
}
