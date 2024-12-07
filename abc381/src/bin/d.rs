use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.iter_mut().for_each(|ai| *ai -= 1);
    let mut set = vec![false; n];
    let mut ans = 0;
    let mut i = 0;
    let mut j = 0;
    loop {
        if i >= n - 1 {
            break;
        }
        if a[i] == a[i + 1] {
            while set[a[i]] {
                set[a[j]] = false;
                j += 2;
            }
            set[a[i]] = true;
            i += 2;
            ans = ans.max(i - j);
        } else if i != j {
            while j != i {
                set[a[j]] = false;
                j += 2;
            }
            i -= 1;
            j -= 1;
        } else {
            i += 1;
            j += 1;
        }
    }
    println!("{}", ans);
}
