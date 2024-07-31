use std::vec;

use proconio::input;

fn tup_add(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    (a.0 + b.0, a.1 + b.1)
}

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }

    let mut grid = vec![vec![false; w]; h];
    let mut pos = (0, 0);
    let dirs = [(0, h - 1), (1, 0), (0, 1), (w - 1, 0)];
    let mut dir_i = 0;

    for _ in 0..n {
        if grid[pos.1][pos.0] {
            dir_i = (dir_i + 4 - 1) % 4;
            grid[pos.1][pos.0] = false;
            pos = tup_add(pos, dirs[dir_i]);
        } else {
            dir_i = (dir_i + 1) % 4;
            grid[pos.1][pos.0] = true;
            pos = tup_add(pos, dirs[dir_i]);
        }
        pos.0 %= w;
        pos.1 %= h;
    }

    for row in grid.iter() {
        for cell in row {
            print!("{}", if *cell { '#' } else { '.' });
        }
        println!();
    }
}
