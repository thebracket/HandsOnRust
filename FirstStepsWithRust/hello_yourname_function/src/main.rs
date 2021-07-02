use std::io::stdin;

fn what_is_your_name() -> String {          // (1)
    let mut your_name = String::new();      // (2)
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name                               // (3)
}

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name(); // (4)
    println!("Hello, {}", name);
}
