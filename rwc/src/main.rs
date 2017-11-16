use std::io;
use std::io::Read;

fn main() {
    println!("{}", io::stdin().bytes().count());
}
