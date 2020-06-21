struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn main() {
    let my_rectangle = Rectangle { height: 10, width: 20 };
     // Use & to lend the rectangle to area, the other way will by destroy and not available
    // in the main scope
    let area = area(&my_rectangle);

    println!("The area is: {}", area);
    println!("The height and width are: {} - {}", my_rectangle.height, my_rectangle.width);
}
