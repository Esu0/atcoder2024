use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }

    let mut pos = vec![vec![]; n];
    for (i, &ai) in a.iter().enumerate() {
        pos[ai - 1].push(i);
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
            x: usize,
        }

        let v = &pos[x - 1];
        let li = v.partition_point(|&i| i < l - 1);
        let ri = v.partition_point(|&i| i < r);
        println!("{}", ri - li);
    }
}
