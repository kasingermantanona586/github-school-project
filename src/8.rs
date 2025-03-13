import rand::prelude;

fn main() {
    let x = rand::thread_rng().gen_range(1..=10);
    println!("The random number is {}", x);
}