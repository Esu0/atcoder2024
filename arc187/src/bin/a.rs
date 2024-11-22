use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        mut a: [u64; n],
    }
    let mut ans = vec![];
    let mut op = |i: usize, arr: &mut [u64]| {
        ans.push(i + 1);
        arr.swap(i, i + 1);
        arr[i] += k;
    };
    if n == 2 {
        if a[0] <= a[1] {
            println!("Yes");
            println!("0");
        } else {
            op(0, &mut a);
            if a[0] <= a[1] {
                println!("Yes");
                println!("1");
                println!("1");
            } else {
                println!("No");
            }
        }
    } else {
        println!("Yes");
        for i in 1..n - 1 {
            while a[i - 1] > a[i] {
                op(i, &mut a);
            }
        }
        while *a.last().unwrap() < a[n - 3] {
            op(n - 2, &mut a);
            op(n - 2, &mut a);
        }
        op(n - 3, &mut a);
        while a[n - 3] > a[n - 2] {
            op(n - 2, &mut a);
            op(n - 2, &mut a);
        }
        println!("{}", ans.len());
        print!("{}", ans[0]);
        ans[1..].iter().for_each(|&a| print!(" {}", a));
        println!();
    }
}
