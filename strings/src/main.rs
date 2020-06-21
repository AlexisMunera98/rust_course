fn main() {
    // str -> Immutable - Stack
    // String -> Mutable - Heap

    // str
    let variable_str = "This is a str";

    // Empty String -> ::new, using str -> ::from
    let mut string_variable = String::from("This is a String");
    string_variable.push(',');
    string_variable.push(' ');
    string_variable.push('N');
    string_variable.push('E');
    string_variable.push('W');

    string_variable.push_str(" a str"); // Is like a "Hello" +  "a str"
    println!("The str is: {}", variable_str);
    println!("The String is: {}", string_variable);

    let new_string = "Hello, I'm a str".to_string();
    println!("The new String is: {}", new_string);

    let equal = new_string == "Another str".to_string();
    println!("Are equals? {}", equal)
}
