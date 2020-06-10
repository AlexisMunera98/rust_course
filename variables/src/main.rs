fn main() {
    // One line comment
    /*
    Multiline comment
     */

    //consts
    const CONSTANT: i32 = 10;

    //variables
    let number_one: i32 = 10; // Fixed type
    let number_two = 20; // Inference type
    let mut other_number: i32; // Mutable variable
    other_number = 123;
    other_number = 43;
    let result = number_one + number_two;

    println!("El valor ({} + {} + {}) es: {}", number_one, number_two, CONSTANT, result)
}
