use proconio::input;

fn main() {
    input! {
        n: usize,
        d: u32,
        xy: [(i64, i64); n],
    }

    let (mut x, mut y) = xy.iter().copied().unzip::<_, _, Vec<_>, Vec<_>>();
    x.sort_unstable();
    y.sort_unstable();
    let m = x[0].abs().max(x[n - 1].abs());
    let offset = m as usize + d as usize;
    let mut x0 = -m - d as i64 - 1;
    let mut dist_sum_x = x.iter().map(|&x| x - x0).sum::<i64>();
    let mut i = 0;
    let mut dist_x = vec![0; 2 * (m as usize + d as usize) + 1];
    while x0 < m + d as i64 {
        let diff = 2 * i as i64 - n as i64;
        dist_sum_x += diff;
        x0 += 1;
        dist_x[offset.wrapping_add_signed(x0 as isize)] = dist_sum_x;
        while i < n && x[i] == x0 {
            i += 1;
        }
    }
    dist_x.sort_unstable();

    let m = y[0].abs().max(y[n - 1].abs());
    let mut y0 = -m - d as i64 - 1;
    let offset = m as usize + d as usize;
    let mut dist_sum_y = y.iter().map(|&y| y - y0).sum::<i64>();
    let mut i = 0;
    let mut dist_y = vec![0; 2 * (m as usize + d as usize) + 1];
    while y0 < m + d as i64 {
        let diff = 2 * i as i64 - n as i64;
        dist_sum_y += diff;
        y0 += 1;
        dist_y[offset.wrapping_add_signed(y0 as isize)] = dist_sum_y;
        while i < n && y[i] == y0 {
            i += 1;
        }
    }
    dist_y.sort_unstable();
    let mut j = dist_y.len();
    let mut ans = 0;
    for &dx in dist_x.iter() {
        while j > 0 && dx + dist_y[j - 1] > d as i64 {
            j -= 1;
        }
        ans += j;
    }
    println!("{}", ans);
}
