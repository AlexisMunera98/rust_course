// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

#[derive(Debug)]
enum ErrorDivision {
    DivisionByZero,
    DivisionNegative,
}

fn division(number1: i32, number2: i32) -> Result<i32, ErrorDivision> {
    if number2 == 0 {
        return Err(ErrorDivision::DivisionByZero);
    }
    if number1 < 0 || number2 < 0 {
        return Err(ErrorDivision::DivisionNegative);
    }
    Ok(number1 / number2)
}

fn main() {
    // Result
    let result = division(10, 0);

    let value = result.unwrap_or(0);
    println!("El result es: {}", value);
    // match result {
    //     Ok(value) => println!("El result es: {}", value),
    //     Err(ErrorDivision::DivisionByZero) => println!("Es una division por cero"),
    //     Err(ErrorDivision::DivisionNegative) => {
    //         panic!("Es una division con numeros negativos")
    //     }
    // }
}
