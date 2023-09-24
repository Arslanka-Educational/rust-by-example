use std::fmt;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> fmt::Display for Person<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.age)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}


fn main() {
    let name = "Arslan";
    let age = 19;
    let person = Person { name, age };

    // Pretty print person with Debug
    println!("{:#?}", person);
    // Pretty print person with Display
    println!("{}", person);

    let list = List(vec![1, 2, 3]);

    //Pretty print Vec with Display
    println!("{}", list);
}
