use proconio::input;

fn main() {
    input! {
        s: [u32; 8],
    }

    if s.windows(2).all(|w| w[0] <= w[1]) && s.iter().all(|&si| (100..=675).contains(&si) && si % 25 == 0) {
        println!("Yes");
    } else {
        println!("No");
    }
}
