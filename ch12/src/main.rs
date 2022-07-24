use std::env;
fn main() {
    println!("Hello, world!");
    // cargo run substring xxx.txt
    let args: Vec<String> = env::args().collect();
}
