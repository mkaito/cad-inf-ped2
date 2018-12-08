extern crate ped2;

fn main() {
    println!("Hola!");

    // Leer dos números de stdin a u32
    let n1 = ped2::leer_numero("Introduzca el primer número");
    let n2 = ped2::leer_numero("Introduzca el segundo número");

    // Escribir un mensaje dependiendo de la comparación entre los dos números
    if n1 == n2 {
        println!("Son iguales!");
    } else {
        println!("El número mayor es {}.",
                 if n1 > n2 { n1 } else { n2 });
    }

    println!("Gracias y hasta luego.");
    ped2::pause();
}
