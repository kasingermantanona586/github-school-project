use std::fs;
fn main() {
    let mut file = fs::File::open("file.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
