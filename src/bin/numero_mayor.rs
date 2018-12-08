use std::io::{self, Write};
extern crate dont_disappear;

fn leer_numero(mensaje: &str) -> u32 {
    let mut buffer = String::new();
    print!("{}: ", mensaje);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer)
        .expect("Ha ocurrido un error al leer stdin.");

    buffer.trim().parse::<u32>()
        .expect("El dato introducido no es un número válido")
}

fn main() {
    println!("Hola!");

    // Leer dos números de stdin a u32
    let n1 = leer_numero("Introduzca el primer número");
    let n2 = leer_numero("Introduzca el segundo número");

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
