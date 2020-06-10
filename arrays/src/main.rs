fn main() {
    let numeros = [1, 2, 3, 4];
    println!("Cosas {:?}", numeros); // Para imprimir listas {:?}

    let numeros: [i32; 5] = [1, 2, 3, 4, 5]; // Definicion explicita
    println!("Cosas {:?}", numeros); // Para imprimir listas {:?}

    // Valores por default
    let valores = [true; 10];
    println!("Lista valores por defecto {:?}", valores); // Para imprimir listas {:?}

    let primer_elemento = numeros[0];
    let ultimo_elemento = numeros[numeros.len() - 1];
    println!("Primero: {}, ultimo {}", primer_elemento, ultimo_elemento)

}
