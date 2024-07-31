use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }
    let mut q = vec![0usize; n];
    for (i, &pi) in p.iter().enumerate() {
        q[pi - 1] = i;
    }
    // eprintln!("{:?}", q);
    let mut op = Vec::new();
    for i in 1..n {
        let qi = q[i];
        let mut l = i;
        for j in (0..i).rev() {
            if q[j] >= qi + k {
                op.push([qi + 1, q[j] + 1]);
                q.swap(l, j);
                l = j;
            }
        }
    }

    // let mut p = p;
    // for &op in &op {
    //     let [l, r] = op;
    //     assert!(p[l - 1] > p[r - 1]);
    //     p.swap(l - 1, r - 1);
    //     eprintln!("{:?}", p);
    // }
    println!("{}", op.len());
    for op in op {
        println!("{} {}", op[0], op[1]);
    }
}
