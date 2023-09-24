#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

use std::fmt;

impl<'a> fmt::Display for Person<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.age)
    }
}

fn main() {
    let name = "Arslan";
    let age = 19;
    let person = Person { name, age };

    // Pretty print Debug
    println!("{:#?}", person);
    // Pretty print Display
    println!("{}", person);
}
