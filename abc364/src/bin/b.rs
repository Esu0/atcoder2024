use proconio::{input, marker};

fn main() {
    input! {
        h: usize,
        w: usize,
        si: usize,
        sj: usize,
        c: [marker::Bytes; h],
        x: marker::Bytes,
    }
    let mut si = si - 1;
    let mut sj = sj - 1;
    for &xi in &x {
        let (next_si, next_sj) = match xi {
            b'U' => (si.wrapping_sub(1), sj),
            b'D' => (si.wrapping_add(1), sj),
            b'L' => (si, sj.wrapping_sub(1)),
            b'R' => (si, sj.wrapping_add(1)),
            _ => unreachable!(),
        };
        if next_si < h && next_sj < w && c[next_si][next_sj] != b'#' {
            (si, sj) = (next_si, next_sj);
        }
    }
    println!("{} {}", si + 1, sj + 1);
}
