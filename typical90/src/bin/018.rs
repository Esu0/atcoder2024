use proconio::input;

fn main() {
    input! {
        t: u32,
        l: u32,
        x: u32,
        y: u32,
        q: usize,
        e: [u32; q],
    }
    let t = t as f64;
    let l = l as f64;
    let x = x as f64;
    let y = y as f64;
    for &ei in &e {
        let theta = std::f64::consts::TAU * ei as f64 / t;
        let r = l / 2.;
        let (si, co) = theta.sin_cos();
        let y1 = -r * si;
        let z1 = r * (1. - co);
        let dy = y - y1;
        let d = x.hypot(dy);
        let ans = (z1 / d).atan().to_degrees();
        println!("{}", ans);
    }
}
