fn main() {

    // Slices -> Heap
    // Arrays -> Stack

    let mensaje = String::from("Hello from Rust course");

    let hola = &mensaje[..5];
    let resto_mensaje = &mensaje[5..];

    println!("El mensaje es: {}", mensaje);
    println!("El mensaje es: {}", hola);
    println!("El resto es: {}", resto_mensaje);
}
