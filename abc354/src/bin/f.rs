use proconio::input;

#[derive(Debug, Clone, Copy)]
enum UndoQuery<T> {
    Pop,
    Update(usize, T)
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            a: [u32; n],
        }

        let mut v = Vec::<u32>::with_capacity(n);
        let mut v2 = Vec::<u32>::with_capacity(n);
        let mut queries = Vec::with_capacity(n);
        for &ai in a.iter().rev() {
            let l = v2.partition_point(|&x| x > ai);
            if l == v2.len() {
                v2.push(ai);
                queries.push(UndoQuery::Pop);
            } else {
                queries.push(UndoQuery::Update(l, v2[l]));
                v2[l] = ai;
            }
        }
        let max_len = v2.len();
        let mut ans = vec![];
        for (i, &ai) in a.iter().enumerate() {
            match queries.pop().unwrap() {
                UndoQuery::Pop => {
                    v2.pop();
                }
                UndoQuery::Update(i, x) => {
                    v2[i] = x;
                }
            }
            // eprintln!("{:?}", v2);
            let l1 = v2.partition_point(|&x| x > ai);
            let l2 = v.partition_point(|&x| x < ai);
            if l1 + l2 + 1 == max_len {
                ans.push(i + 1);
            }
            if l2 == v.len() {
                v.push(ai);
            } else {
                v[l2] = ai;
            }
        }
        println!("{}", ans.len());
        for &a in &ans {
            print!("{} ", a);
        }
        println!();
    }
}
