use proconio::input;

const MAX: usize = 1000;
fn main() {
    input! {
        n: usize,
        lxlyrxry: [(usize, usize, usize, usize); n],
    }

    let mut map = vec![vec![0; MAX + 1]; MAX + 1];

    for &(lx, ly, rx, ry) in &lxlyrxry {
        map[ly][lx] += 1;
        map[ly][rx] -= 1;
        map[ry][lx] -= 1;
        map[ry][rx] += 1;
    }
    for row in &mut map {
        for x in 1..=MAX {
            row[x] += row[x - 1];
        }
    }

    for x in 0..=MAX {
        for y in 1..=MAX {
            map[y][x] += map[y - 1][x];
        }
    }
    // eprintln!("{:?}", map);
    let mut ans = vec![0; n];
    for row in &map[..MAX] {
        for &cell in &row[..MAX] {
            if cell != 0 {
                ans[(cell - 1) as usize] += 1;
            }
        }
    }
    for &a in &ans {
        println!("{}", a);
    }
}
