use std::collections::HashMap;

fn main() {
    let mut students = HashMap::new();
    students.insert("Alice".to_string(), 28);
    students.insert("Bob".to_string(), 30);
    students.insert("Charlie".to_string(), 25);
    
    for (student, age) in &students {
        println!("{} is {} years old.", student, age);
    }
}
