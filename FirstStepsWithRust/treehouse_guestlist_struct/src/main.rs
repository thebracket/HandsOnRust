use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {    // (1)
    fn new(name: &str, greeting: &str) -> Self {    // (2)
        Self {  // (3)
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) { // (4)
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    let visitor_list = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve","Hi Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];

    println!("Hello, what's your name?");
    let name = what_is_your_name();

    let known_visitor = visitor_list// (5)
        .iter()// (6)
        .find(|visitor| visitor.name == name);// (7)
    match known_visitor {// (8)
        Some(visitor) => visitor.greet_visitor(),// (9)
        None => println!("You are not on the visitor list. Please leave.")// (10)
    }
}
