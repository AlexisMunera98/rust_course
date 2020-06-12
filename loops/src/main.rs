fn main() {
    // Loop
    let mut counter = 0;
    loop { // This is a infinite loop, like while true
        counter += 1;
        println!("Counter {}", counter);
        if counter >= 5 {
            break;
        }
    }

    println!("For loop");

    // for
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    for number in numbers.iter() {
        println!("The number is {}", number)
    }

    for item in 1..10 { // This is a range like range(1, 10) in python
        println!("The item is {}", item)
    }

    // Fizz Buzz

    for number in 1..16 {
        if number % 3 == 0 && number % 5 == 0 {
            println!("Fizz Buzz");
        } else if number % 3 == 0 {
            println!("Fizz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", number)
        }
    }

    // While loop
    // let mut counter = 1;
    // while counter <= 10 {
    //     println!("Counter {}", counter);
    //     counter += 1;
    // }

    let mut number = 123;
    let mut counter = 0;
    while number > 0 {
        number = number / 10;
        counter += 1;
    }
    println!("The amount of digits are: {}", counter)
}
