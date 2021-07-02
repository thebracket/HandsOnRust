use std::io::stdin;

#[derive(Debug)]    // (1)
enum VisitorAction {  // (2)
  Accept,         // (3)
  AcceptWithNote { note: String },    // (4)
  Refuse,
  Probation,
} // (5)


#[derive(Debug)]
struct Visitor {
  name: String,
  action: VisitorAction,    // (6)
  age: i8                // (7)
}

impl Visitor {
  fn new(name: &str, action: VisitorAction, age: i8) -> Self { // (8)
    Self {
      name: name.to_lowercase(), // (9)
      action, // (10)
      age
    }
  }
  fn greet_visitor(&self) {
    match &self.action {
      VisitorAction::Accept => println!("Welcome to the tree 
          house, {}", self.name), // (11)
      VisitorAction::AcceptWithNote { note } => { // (12)
          println!("Welcome to the treehouse, {}", self.name);
          println!("{}", note); // (13)
          if self.age < 21 { // (14)
              println!("Do not serve alcohol to {}", self.name);
          }
      }
      VisitorAction::Probation => println!("{} is now a 
          probationary member", self.name),
      VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
      }
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
  let mut visitor_list = vec![
    Visitor::new("Bert", VisitorAction::Accept, 45),
    Visitor::new("Steve", VisitorAction::AcceptWithNote{ 
        note: String::from("Lactose-free milk is in the fridge") 
    }, 15),
    Visitor::new("Fred", VisitorAction::Refuse, 30),
  ];
  loop {
    println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
    let name = what_is_your_name();
    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
    match known_visitor {
      Some(visitor) => visitor.greet_visitor(),
      None => {
        if name.is_empty() {
          break;
        } else {
          println!("{} is not on the visitor list.", name);
          visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
        }
      }
    }
  }
  println!("The final list of visitors:");
  println!("{:#?}", visitor_list);
}
