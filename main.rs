fn main() {
    let message = "Hello World";
    print_welcome(message);
}

fn print_welcome(text: &str) -> &str {
    println!("{}", text);
    let new_message = "New Message";
    return new_message
}

// Hello World
