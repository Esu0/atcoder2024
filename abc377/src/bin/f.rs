use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: i64,
        m: usize,
        ab: [(i64, i64); m],
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum Line {
        // x = a
        Vertical(i64),
        // y = a
        Horizontal(i64),
        // x + y = b
        DiagonalPos(i64),
        // x - y = b
        DiagonalNeg(i64),
    }

    use Line::*;
    let mut lines = ab
        .iter()
        .flat_map(|&(a, b)| {
            [
                Horizontal(a),
                Vertical(b),
                DiagonalPos(a + b),
                DiagonalNeg(b - a),
            ]
        })
        .collect::<Vec<_>>();
    lines.sort_unstable();
    lines.dedup();
    let intersec = |l1: Line, l2: Line| {
        // l1 != l2
        if std::mem::discriminant(&l1) == std::mem::discriminant(&l2) {
            return (0, 0);
        }
        match (l1, l2) {
            (Horizontal(a), Vertical(b)) | (Vertical(b), Horizontal(a)) => (b, a),
            (Horizontal(a), DiagonalPos(b)) | (DiagonalPos(b), Horizontal(a)) => (b - a, a),
            (Horizontal(a), DiagonalNeg(b)) | (DiagonalNeg(b), Horizontal(a)) => (b + a, a),
            (Vertical(a), DiagonalPos(b)) | (DiagonalPos(b), Vertical(a)) => (a, b - a),
            (Vertical(a), DiagonalNeg(b)) | (DiagonalNeg(b), Vertical(a)) => (a, a - b),
            (DiagonalPos(a), DiagonalNeg(b)) | (DiagonalNeg(b), DiagonalPos(a)) => {
                if (a + b) % 2 == 0 {
                    ((a + b) / 2, (a - b) / 2)
                } else {
                    (0, 0)
                }
            }
            _ => unreachable!(),
        }
    };

    // eprintln!("{:?}", lines);
    let mut intersections = HashSet::new();
    let mut intersec_count = 0;
    for (i, &l1) in lines.iter().enumerate() {
        intersections.clear();
        for &l2 in &lines[i + 1..] {
            let (x, y) = intersec(l1, l2);
            if (1..=n).contains(&x) && (1..=n).contains(&y) {
                // if matches!(l1, DiagonalPos(_)) {
                //     eprintln!("{l1:?} {} {}", x, y);
                // }
                intersections.insert((x, y));
            }
        }
        intersec_count += intersections.len() as u64;
    }

    let unavail = lines
        .iter()
        .map(|&line| {
            (match line {
                Horizontal(_) | Vertical(_) => n,
                DiagonalPos(a) => {
                    if a <= n + 1 {
                        a - 1
                    } else {
                        2 * n - a + 1
                    }
                }
                DiagonalNeg(a) => n - a.abs(),
            }) as u64
        })
        .sum::<u64>();
    let n = n as u64;
    let ans = n * n - (unavail - intersec_count);
    println!("{}", ans);
}
