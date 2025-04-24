use std::fs;

// Define a function to read a file and print its contents.
fn main() {
    let file_path = "path/to/your_file.txt"; // Replace with your file path

    match fs::read_to_string(file_path) {
        Ok(content) => println!("{}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
