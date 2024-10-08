use std::sync::atomic::{AtomicU32, Ordering::*};

const R: u64 = 998244353;
const N: u64 = 1000000007;

const fn mpow(base: u32, mut exp: u64, modulo: u32) -> u32 {
    let mut base = base as u64;
    let modulo = modulo as u64;
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulo;
        }
        base = base * base % modulo;
        exp >>= 1;
    }
    result as u32
}

const R_INV: u64 = {
    R.wrapping_pow(1 << 31).wrapping_pow(2).wrapping_pow(u32::MAX >> 1).wrapping_mul(R.wrapping_pow(u32::MAX))
};
const N_P: u64 = (R - mpow(N as u32, R - 2, R as u32) as u64) % R;

const R2: u64 = R * R % N;
const N_NEG: u64 = N.wrapping_neg();
const N_NEG_2: u64 = N_NEG.wrapping_add(N_NEG);
const NEG_ONE: u64 = 1u64.wrapping_neg();

static COUNT: AtomicU32 = AtomicU32::new(0);
fn add(a: u64, b: u64) -> u64 {
    COUNT.fetch_add(1, Relaxed);
    a.wrapping_add(b)
}

fn mul(a: u64, b: u64) -> u64 {
    COUNT.fetch_add(1, Relaxed);
    a.wrapping_mul(b)
}

fn rem(a: u64) -> u64 {
    COUNT.fetch_add(1, Relaxed);
    a % R
}

fn div_floor_r(a: u64) -> u64 {
    mul(add(a, mul(rem(a), NEG_ONE)), R_INV)
}

fn montgomery(a: u64) -> u64 {
    mul(add(a, mul(rem(mul(rem(a), N_P)), N)), R_INV)
}

fn solve(a: u64, b: u64) -> u64 {
    let ab = mul(a, b);
    let ma = montgomery(ab);
    let ma = add(ma, N_NEG_2);
    let c = div_floor_r(div_floor_r(ma));
    let c = add(c, R - 1);
    let c = div_floor_r(c);
    let ma = add(ma, mul(c, N));
    let c = div_floor_r(div_floor_r(ma));
    let c = div_floor_r(add(c, R - 1));
    let ma = add(ma, mul(c, N));
    let a = montgomery(mul(ma, R2));
    let a = add(a, N_NEG_2);
    let c = div_floor_r(div_floor_r(a));
    let c = div_floor_r(add(c, R - 1));
    let a = add(a, mul(c, N));
    let c = div_floor_r(div_floor_r(a));
    let c = div_floor_r(add(c, R - 1));
    add(a, mul(c, N))
}

use proconio::input;
use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    input! {
        n: usize,
    }
    let mut registers = [0; 26];
    registers[0] = rng.gen_range(0..N);
    // registers[1] = rng.gen_range(0..N);
    let ans = registers[0] * registers[1] % N;
    for _ in 0..n {
        input! {
            op: String,
        }
        match op.as_str() {
            "add" => {
                input! {
                    x: char,
                    y: String,
                    z: String,
                }
                let x = x as usize - 'A' as usize;
                let y = if y.len() == 1 {
                    registers[(y.as_bytes()[0] - b'A') as usize]
                } else {
                    y.parse::<u64>().unwrap()
                };
                let z = if z.len() == 1 {
                    registers[(z.as_bytes()[0] - b'A') as usize]
                } else {
                    z.parse::<u64>().unwrap()
                };
                registers[x] = add(y, z);
            }
            "mul" => {
                input! {
                    x: char,
                    y: String,
                    z: String,
                }
                let x = x as usize - 'A' as usize;
                let y = if y.len() == 1 {
                    registers[(y.as_bytes()[0] - b'A') as usize]
                } else {
                    y.parse::<u64>().unwrap()
                };
                let z = if z.len() == 1 {
                    registers[(z.as_bytes()[0] - b'A') as usize]
                } else {
                    z.parse::<u64>().unwrap()
                };
                registers[x] = mul(y, z);
            }
            "rem" => {
                input! {
                    x: char,
                    y: String,
                }
                let x = x as usize - 'A' as usize;
                let y = if y.len() == 1 {
                    registers[(y.as_bytes()[0] - b'A') as usize]
                } else {
                    y.parse::<u64>().unwrap()
                };
                registers[x] = rem(y);
            }
            "print" => {
                eprintln!("{:?}", registers);
            }
            _ => unreachable!(),
        }
    }
    assert_eq!(ans, registers[2]);
}
