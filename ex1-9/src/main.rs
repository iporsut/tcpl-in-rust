use std::io;
use std::io::Read;
use std::io::Write;

fn main() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buf: [u8; 1] = [0; 1];
    let mut in_space = false;

    loop {
        match stdin.read(&mut buf) {
            Ok(1)  => {
                if in_space && buf[0] == ' ' as u8 {
                    continue
                }

                stdout.write(&buf).unwrap();

                if !in_space && buf[0] == ' ' as u8 {
                    in_space = true;
                } else if in_space && buf[0] != ' ' as u8 {
                    in_space = false;
                }

            },
            _ => {
                return;
            }
        }
    }
}
