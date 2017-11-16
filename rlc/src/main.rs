use std::io;
use std::io::Read;

fn main() {
    let mut nl = 0;
    for c in io::stdin().bytes() {
        if c.unwrap() == '\n' as u8 {
            nl += 1;
        }
    }
    println!("{}", nl);
}
