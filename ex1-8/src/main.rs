use std::io;
use std::io::Read;

struct Count { b: i32, t: i32, n: i32 }
impl Count {
    fn def() -> Count {
        Count{b: 0, t: 0, n: 0}
    }

    fn b_inc(self) -> Count {
        Count{b: self.b + 1, .. self}
    }

    fn t_inc(self) -> Count {
        Count{t: self.t + 1, .. self}
    }

    fn n_inc(self) -> Count {
        Count{n: self.n + 1, .. self}
    }
}

fn main() {
    let c = io::stdin()
        .bytes()
        .fold(Count::def(), |c, r| match r {
            Ok(b)  => match b {
                b' '  => c.b_inc(),
                b'\t' => c.t_inc(),
                b'\n' => c.n_inc(),
                _    => c
            },
            _      => c
        });

    println!("{} {} {}", c.b, c.t, c.n);
}
