use proconio::input;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Float(f64);

impl Eq for Float {}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut angles = Vec::with_capacity(n - 1);
    let mut angles2 = Vec::with_capacity((n - 1) * 2);
    let mut ans = 0.0f64;
    for i in 0..n {
        let (p, s) = xy.split_at(i);
        let (x0, y0) = s[0];
        // buf.extend_from_slice(p);
        // buf.extend_from_slice(&s[1..]);
        angles.clear();
        angles.extend(p.iter().copied().chain(s[1..].iter().copied()).map(|(x, y)| {
            let dx = (x - x0) as f64;
            let dy = (y - y0) as f64;
            dy.atan2(dx)
        }));
        angles.sort_unstable_by_key(|&a| Float(a));
        angles2.clear();
        angles2.extend_from_slice(&angles);
        angles2.extend(angles.iter().map(|&a| a + std::f64::consts::TAU));

        let mut i = 0;
        let mut j = 0;
        let mut max = 0.0f64;
        let l = angles2.len();
        while i < l {
            while j < l && angles2[j] - angles2[i] < std::f64::consts::PI {
                j += 1;
            }
            max = max.max(angles2[j - 1] - angles2[i]);
            if j < l {
                max = max.max(std::f64::consts::TAU - (angles2[j] - angles2[i]));
            }
            i += 1;
        }
        ans = ans.max(max);
    }
    println!("{}", ans.to_degrees());
}
