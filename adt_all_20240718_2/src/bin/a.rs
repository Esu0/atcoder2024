use proconio::input;

fn main() {
    input! {
        s: [String]
    }
    for si in s.iter().rev() {
        println!("{}", si.as_str());
    }
}
