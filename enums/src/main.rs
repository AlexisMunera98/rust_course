fn main() {
    // Enum -> Camelcase
    enum Response {
        Success,
        Error(u32), // 403, 404, 500
    }

    let response = Response::Error(200);

    match response {
        Response::Success => println!("The request has successful"),
        Response::Error(403) => println!("The request forbidden"),
        Response::Error(404) => println!("The request not found"),
        Response::Error(500) => println!("The request Internal server error"),
        Response::Error(_) => println!("Unknown error"),
    }
}
