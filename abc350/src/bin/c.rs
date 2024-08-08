use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    let mut pos = vec![usize::MAX; n];
    a.iter().enumerate().for_each(|(i, &ai)| pos[ai - 1] = i);
    // println!("{}", n - 1);
    let mut op = Vec::new();
    for i in 0..n - 1 {
        if i != pos[i] {
            op.push((i + 1, pos[i] + 1));
        }
        let ai = a[i];
        a.swap(i, pos[i]);
        pos.swap(ai - 1, i);
    }
    println!("{}", op.len());
    for (i, j) in op {
        println!("{} {}", i, j);
    }
}
