use proconio::{input, marker};

fn rot_frame(grid: &mut [Vec<u8>], offset: usize, n: usize, buf: &mut [u8]) {
    buf[..n - 1].copy_from_slice(&grid[offset][offset..offset + n - 1]);
    for i in 0..n - 1 {
        grid[offset][offset + i] = grid[offset + n - 1 - i][offset];
    }
    for i in 1..n {
        grid[offset + i][offset] = grid[offset + n - 1][offset + i];
    }
    for i in 1..n {
        grid[offset + n - 1][offset + i] = grid[offset + n - 1 - i][offset + n - 1];
    }
    for i in 0..n - 1 {
        grid[offset + i][offset + n - 1] = buf[i];
    }
}

fn main() {
    input! {
        n: usize,
        mut a: [marker::Bytes; n],
    }
    assert!(n % 2 == 0);
    let mut buf = vec![0; n - 1];
    for i in 0..n / 2 {
        let offset = i;
        let n = n - 2 * i;
        let count = (i + 1) % 4;
        for _ in 0..count {
            rot_frame(&mut a, offset, n, &mut buf);
        }
    }
    for row in &a {
        println!("{}", std::str::from_utf8(row).unwrap());
    }
}
