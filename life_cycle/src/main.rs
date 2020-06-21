fn main() { // Block 1

    // Block limita el scope de una variable
    let mensaje = "Hola, soy una variable del bloque main";

    println!("Variable: {}", mensaje);

    { // Block 2
        let mensaje = "Hola, soy una variable del bloque 2";

        println!("Variable: {}", mensaje);

        { // Block 3
            println!("Variable: {}", mensaje);
            let result = 10 + 20;
        }
    }

    let mut other_message = String::from("Una variable para prestamo");
    {
        let prestamo = &other_message; // Borrow -> move
        other_message = String::from("Cambio de valor");
        println!("{}", other_message);
    }
    println!("{}", other_message);
}
