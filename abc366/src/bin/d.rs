use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [[[u64; n]; n]; n],
        q: usize,
    }

    let mut b = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                b[i + 1][j + 1][k + 1] = a[i][j][k];
            }
        }
    }
    // ゼータ変換
    for bi in &mut b {
        for bij in bi {
            let mut sum = 0;
            for bijk in bij {
                sum += *bijk;
                *bijk = sum;
            }
        }
    }

    for bi in &mut b {
        for k in 0..=n {
            let mut sum = 0;
            for bij in &mut *bi {
                sum += bij[k];
                bij[k] = sum;
            }
        }
    }

    for j in 0..=n {
        for k in 0..=n {
            let mut sum = 0;
            for bi in &mut b {
                sum += bi[j][k];
                bi[j][k] = sum;
            }
        }
    }

    // eprintln!("{:?}", b);
    for _ in 0..q {
        input! {
            lx: usize,
            rx: usize,
            ly: usize,
            ry: usize,
            lz: usize,
            rz: usize,
        }
        let lx = lx - 1;
        let ly = ly - 1;
        let lz = lz - 1;
        let ans = b[rx][ry][rz] + b[lx][ly][rz] + b[lx][ry][lz] + b[rx][ly][lz]
            - b[lx][ly][lz]
            - b[lx][ry][rz]
            - b[rx][ly][rz]
            - b[rx][ry][lz];
        println!("{}", ans);
    }
}
