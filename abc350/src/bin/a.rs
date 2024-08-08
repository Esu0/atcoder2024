use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let n = s[3..].parse::<u32>().unwrap();
    if (1..=349).contains(&n) && n != 316 {
        println!("Yes");
    } else {
        println!("No");
    }
}
