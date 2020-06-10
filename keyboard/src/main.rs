use std::io;

fn main() {
    let mut username = String::new();
    println!("Ingresa el nombre de usuario");
    // Esto retorna un Result -> Success or Error
    io::stdin().read_line(&mut username);

    let username = username.trim();

    let mut edad = String::new();
    println!("Ingresa la edad del usuario");
    io::stdin().read_line(&mut edad);

    let edad = edad.trim();
    let edad: i32 = edad.parse().unwrap();
    println!("El nombre del usuario es {}", username);
    println!("La edad es {}", edad)
}
