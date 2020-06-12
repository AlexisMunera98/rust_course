fn main() {
    let number: i32 = 55;

    let message = match number {
        1 => "The number is one",
        2 => "The number is two",
        3 => "The number is three",
        4 | 5 | 6 => "Is between 4 and 6",
        7..=100 => {
            println!("Aqui pasan cosas");
            "Esta en un rango"
        }
        _ => "Is other number"
    };
    println!("{}", message);

    // Fizz Buzz with match
    for number in 1..=30 {
        match (number % 3, number % 5) {
            (0, 0) => println!("Fizz Buzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", number)
        }
    }
}
