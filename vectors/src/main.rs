fn main() {
    // Usando vec!
    // let mut vector = vec![1, 2, 3];

    // Usando struct Vec
    let mut vector = Vec::new();



    println!("El valor del vector es {:?}", vector);
    vector.push(4);
    vector.push(5);
    println!("El valor del vector es {:?}", vector);
    vector.insert(0, 99);
    println!("El valor del vector es {:?}", vector);
    vector.remove(vector.len() - 1); // Remove last element
    println!("El valor del vector es {:?}", vector);

    let primer_elemento = vector[0];
    // let ultimo_elemento = vector[vector.len() - 1];
    let ultimo_elemento = vector.pop().unwrap(); //Option

    println!("Primer: {}, Ultimo: {}", primer_elemento, ultimo_elemento);



}
