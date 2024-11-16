use proconio::{input, marker};

fn main() {
    input! {
        _n: usize,
        mut s: marker::Bytes,
        q: usize,
    }

    let mut map = [u8::MAX; 26];
    for (i, mapi) in map.iter_mut().enumerate() {
        *mapi = b'a' + i as u8;
    }

    for _ in 0..q {
        input! { c: char, d: char }
        let c = c as u8;
        let d = d as u8;
        for mapi in map.iter_mut().filter(|&&mut mapi| mapi == c) {
            *mapi = d;
        }
    }

    for si in s.iter_mut() {
        *si = map[*si as usize - b'a' as usize];
    }
    println!("{}", std::str::from_utf8(&s).unwrap());
}
