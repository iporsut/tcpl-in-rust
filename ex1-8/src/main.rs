use std::io;
use std::io::Read;

fn main() {
    let (bc, tc, nc) = io::stdin()
        .bytes()
        .fold((0, 0, 0), |acc, r| {
            let (bc, tc, nc) = acc;
            match r {
                Ok(c)  => match c as char {
                    ' '  => (bc + 1, tc, nc),
                    '\t' => (bc, tc + 1, nc),
                    '\n' => (bc, tc, nc + 1),
                    _    => acc
                },
                _      => acc

            }
        });

    println!("{} {} {}", bc, tc, nc);
}
