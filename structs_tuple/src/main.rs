use std::thread::current;

#[derive(Debug)]
struct Color(u32, u32, u32);// RGB

fn main() {
    let black = Color(0, 0, 0);
    let white = Color(225, 225, 225);

    let mut custom_color = Color(187, 62, 104);
    custom_color.1 = custom_color.1 + 10;

    println!("El color es: {:?}", black);
    println!("El color es: {:?}", white);
    println!("El color es: {:?}", custom_color);
}
