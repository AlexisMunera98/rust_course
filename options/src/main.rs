// enum Option<T>{
//     Some(T), -> El valor
//     None, -> La ausencia del valor
// }

#[derive(Debug)]
struct User {
    username: String,
    password: String,
    email: String,
    age: Option<u32>,
}

fn get_value(flag: bool) -> Option<String> {
    if flag {
        Some(String::from("A message to some tuple"))
    } else {
        None
    }
}

fn foo(possibly_supplied: Option<usize>) -> usize {
    possibly_supplied.unwrap_or(20) //Default value is 20
}


fn main() {
    // Option -> Si existe o no algun valor
    // Result -> Manejo de errores -> panic!

    let result = get_value(true);
    // match result {
    //     Some(value) => println!("El resultado fue {}", value),
    //     None => println!("No hubo valor")
    // }

    // unwrap -> Intenta  obtener el valor de la tupla some
    // let valor = result.unwrap_or("No hay nada".to_string());
    let valor = result.expect("Se esperaba un string");
    println!("El valor es: {}", valor);

    let s = foo(Some(2));
    println!("{}", s);


    let user1 = User {
        username: String::from("Alexis"),
        password: String::from("pass"),
        email: String::from("correo@email.com"),
        age: Some(21),
    };

    // let age = user1.age.unwrap();
    println!("{:?}", user1);
    match user1.age {
        Some(age) => println!("The age is {}", age),
        None => {}
    }
}
