use std::io;
use std::io::Read;
use std::io::Write;

fn main() {
    let mut stdin    = io::stdin();
    let mut stdout   = io::stdout();
    let mut buf      = [0 as u8];
    let mut in_space = false;

    while let Ok(1) = stdin.read(&mut buf) {
        if in_space && buf[0] == b' ' {
            continue
        }

        stdout.write(&buf).unwrap();

        if !in_space && buf[0] == b' ' {
            in_space = true;
        } else if in_space && buf[0] != b' ' {
            in_space = false;
        }
    }
}
