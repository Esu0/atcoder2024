use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        mut s: marker::Bytes,
    }
    let mut dp = vec![[i32::MIN; 3]; n + 1];
    dp[1] = [0, 0, 0];
    match s[0] {
        b'R' => {
            dp[1][1] = i32::MIN;
            dp[1][2] = 1;
        }
        b'S' => {
            dp[1][2] = i32::MIN;
            dp[1][0] = 1;
        }
        b'P' => {
            dp[1][0] = i32::MIN;
            dp[1][1] = 1;
        }
        _ => unreachable!(),
    };
    s.iter_mut().for_each(|c| match *c {
        b'R' => *c = 0,
        b'S' => *c = 1,
        b'P' => *c = 2,
        _ => unreachable!(),
    });
    for i in 2..=n {
        let prev = dp[i - 1];
        // match s[i - 1] {
        //     0 => prev[1] = i32::MIN,
        //     1 => prev[2] = i32::MIN,
        //     2 => prev[0] = i32::MIN,
        //     _ => unreachable!(),
        // };
        dp[i][0] = prev[1].max(prev[2]);
        dp[i][1] = prev[2].max(prev[0]);
        dp[i][2] = prev[0].max(prev[1]);
        match s[i - 1] {
            0 => {
                dp[i][2] += 1;
                dp[i][1] = i32::MIN;
            }
            1 => {
                dp[i][0] += 1;
                dp[i][2] = i32::MIN;
            }
            2 => {
                dp[i][1] += 1;
                dp[i][0] = i32::MIN;
            }
            _ => unreachable!(),
        };
    }
    // eprintln!("{:?}", dp);
    let ans = *dp[n].iter().max().unwrap();
    println!("{ans}");
}
