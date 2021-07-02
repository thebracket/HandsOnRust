use std::io::stdin;

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name
}

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();
    println!("{:?}", name);
}
