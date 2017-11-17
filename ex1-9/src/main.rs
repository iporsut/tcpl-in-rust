use std::io;
use std::io::Read;
use std::io::Write;

fn main() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buf: [u8; 1] = [0; 1];
    let mut in_space = false;
    const BLANK: u8 = ' ' as u8;

    loop {
        if let Ok(1) = stdin.read(&mut buf) {
            if in_space && buf[0] == BLANK {
                continue
            }

            if let Err(_) = stdout.write(&buf) {
                return
            }

            if !in_space && buf[0] == BLANK {
                in_space = true;
            } else if in_space && buf[0] != BLANK {
                in_space = false;
            }

            continue
        }

        return
    }
}
