use std::io;
use std::io::Read;

fn main() {
    println!("{}", io::stdin()
             .bytes()
             .map(|r| match r.map(|c| c == '\n' as u8) {
                 Ok(true) => 1,
                 _        => 0
             })
             .sum::<i32>());
}
