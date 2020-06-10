use std::ptr::read_unaligned;

fn main() {
    println!("Hello, world!");
    let numero_uno: i32 = 10;
    let numero_dos: i32 = 200;

    // + - / * %
    let resultado = numero_dos * numero_uno;
    println!("El resultado {}", resultado);

    // > < >= <= == !=
    let resultado = numero_uno != 2000;
    println!("El resultado {}", resultado);

    // && ||
    let resultado = numero_uno > 100 && true;
    println!("El resultado {}", resultado);
}
