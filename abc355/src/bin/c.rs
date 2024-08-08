use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; t],
    }
    let mut count_row = vec![0; n];
    let mut count_col = vec![0; n];
    let mut count_diag = (0, 0);
    for (t, &ai) in a.iter().enumerate() {
        let i = (ai - 1) / n + 1;
        let j = (ai - 1) % n + 1;
        let n1 = count_row[i - 1] + 1;
        let n2 = count_col[j - 1] + 1;
        if n1 == n || n2 == n {
            println!("{}", t + 1);
            return;
        }
        count_row[i - 1] = n1;
        count_col[j - 1] = n2;
        if i == j {
            count_diag.0 += 1;
        }
        if i + j == n + 1 {
            count_diag.1 += 1;
        }
        if count_diag.0 == n || count_diag.1 == n {
            println!("{}", t + 1);
            return;
        }
    }
    println!("-1");
}
