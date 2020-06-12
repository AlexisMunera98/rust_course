fn sum(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn factorial(number: u32) -> u32 {
    if number == 1 {
        return number;
    } else {
        factorial(number - 1) * number
    }
}

fn main() {
    let result = factorial(5);
    println!("The factorial is {}", result)
}
