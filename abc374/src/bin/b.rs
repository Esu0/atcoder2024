use proconio::{input, marker};

fn main() {
    input! {
        mut s: marker::Bytes,
        mut t: marker::Bytes,
    }

    if s == t {
        println!("0");
        return;
    }

    let mut d = s.len() as i32 - t.len() as i32;
    while d > 0 {
        t.push(0);
        d -= 1;
    }
    while d < 0 {
        s.push(0);
        d += 1;
    }

    for (i, (&si, &ti)) in s.iter().zip(&t).enumerate() {
        if si != ti {
            println!("{}", i + 1);
            break;
        }
    }
}
