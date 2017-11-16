use std::io;
use std::io::Read;

fn main() {
    let mut bc = 0;
    let mut nc = 0;
    let mut tc = 0;

    for r in io::stdin().bytes() {
        let c = r.unwrap();
        match c as char {
            '\n' => { nc += 1; },
            '\t' => { tc += 1; },
            ' '  => { bc += 1; },
            _    => {}
        }
    }

    println!("{} {} {}", bc, tc, nc);
}
