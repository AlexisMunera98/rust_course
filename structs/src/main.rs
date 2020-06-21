use std::io::stdin;

#[derive(Debug)]
struct User {
    username: String,
    password: String,
}

// impl is the implementation of the methods to User struct
impl User {
    // &mut self it's the reference to the struct
    fn greeting(&mut self) {
        println!("Hello, I'm {}", self.username)
    }

    fn change_password(&mut self, new_password: String) {
        self.password = new_password;
    }
}

fn main() {
    let username = String::from("alexis");
    let password = String::from("weak_password");
    let mut new_user = User {
        password, // Is not positional, will infer the value of the attributes taking into
        username, // account the name of the variable
    };

    let mut name =  String::new();
    stdin().read_line(&mut name);
    println!("The input was: {}", name);

    println!("The username is: {}", new_user.username);
    println!("The password is: {}", new_user.password);

    new_user.greeting();
    new_user.change_password("Stronger_password".to_string());
    println!("The new user is: {:?}", new_user);
}
