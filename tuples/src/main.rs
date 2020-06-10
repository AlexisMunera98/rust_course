fn main() {
    let tupla = (10, false, 5.5);
    println!("El valor de la tupla es: {:?}", tupla);

    let tupla: (i32, bool, f64) = (10, false, 5.5);
    println!("El valor de la tupla es: {:?}", tupla);

    // Se acceden a los indices usando dot notation
    let primer_elemento = tupla.0;
    let ultimo_elemento = tupla.2;
    println!("Primer {}, Ultimo: {}", primer_elemento, ultimo_elemento)

}
