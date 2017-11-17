use std::io;
use std::io::BufRead;
fn main() {
    let stdin = io::stdin();
    println!("{}", stdin.lock().lines().count());
}
