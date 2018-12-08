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

pub fn pause() {
    print!("Presione Enter para salir...");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read(&mut [0]).unwrap();
}
