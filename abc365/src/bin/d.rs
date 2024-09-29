use proconio::{input, marker};

fn main() {
    input! {
        _n: usize,
        mut s: marker::Bytes,
    }
    s.iter_mut().for_each(|c| {
        *c = match *c {
            b'R' => 0,
            b'S' => 1,
            b'P' => 2,
            _ => unreachable!(),
        }
    });
    let mut dpi = [0; 3];
    let mut dpi_next;
    for &si in &s {
        dpi_next = [i32::MIN; 3];
        let w = ((si + 2) % 3) as usize;
        dpi_next[w] = dpi[(w + 1) % 3].max(dpi[(w + 2) % 3]) + 1;
        let d = si as usize;
        dpi_next[d] = dpi[(d + 1) % 3].max(dpi[(d + 2) % 3]);
        dpi = dpi_next;
    }
    let ans = dpi.iter().copied().max().unwrap();
    println!("{ans}");
}
