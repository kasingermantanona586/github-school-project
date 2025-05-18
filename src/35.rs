use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "input.txt"; // Replace with your input file path

    if !file_path.exists() {
        println!("The file does not exist: {}", file_path);
        return Ok(());
    }

    let reader = File::open(file_path)?;
    let reader_buf_reader = BufReader::new(reader);

    let mut lines = Vec::<&str>::from_iter(reader_buf_reader.lines());

    // Assume 'input.txt' is the correct path to your input file
    for line in &lines {
        println!("{}", line);
    }

    Ok(())
}
