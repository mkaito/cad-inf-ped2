/*
Copyright © 2018 Christian Höppner

This file is part of the class assignment PED2 for CAD 2018/19.

PED2 is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

PED2 is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Foobar.  If not, see <https://www.gnu.org/licenses/>.
 */

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
