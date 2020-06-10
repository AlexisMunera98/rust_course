use term::color::YELLOW;

fn main() {
    let valor: i32 = 10; // Variable inmutable
    println!("The value of the variable is {}", valor);

    let valor: i32 = 20;  // Se destruye la variable de arriba y se crea nuevamente "Shadowing"
    println!("The value of the variable is {}", valor);

    let valor = false;
    println!("The value of the variable is {}", valor);
}
