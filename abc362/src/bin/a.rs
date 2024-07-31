use proconio::{input, marker};

fn main() {
    input! {
        r: u32,
        g: u32,
        b: u32,
        c: marker::Bytes,
    }

    match c[0] {
        b'R' => println!("{}", g.min(b)),
        b'G' => println!("{}", r.min(b)),
        b'B' => println!("{}", r.min(g)),
        _ => unreachable!(),
    }
}
