extern crate dont_disappear;

fn main() {
    println!("Hola!");

    println!("Alumno: Christian Markwart Höppner.");
    println!("Asignatura: Fundamentos de la Informática.");
    println!("Curso: CAD 2018/19.");

    println!("Gracias y hasta luego.");
    dont_disappear::any_key_to_continue::custom_msg("Presione cualquier tecla para continuar...");
}
