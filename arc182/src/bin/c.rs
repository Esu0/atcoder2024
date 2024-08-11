use proconio::input;

const MODULO: u64 = 998244353;

fn matrix_pow(matrix: &mut [Vec<u64>], n: u64) -> Vec<Vec<u64>> {
    let mat_len = matrix.len();
    let mut result = vec![vec![0u64; mat_len]; mat_len];
    for (i, row) in result.iter_mut().enumerate() {
        row[i] = 1;
    }
    let mut n = n;
    while n > 0 {
        if n & 1 != 0 {
            // result = result * matrix
            for (i, rowi) in result.iter_mut().enumerate() {
                for j in (i..mat_len).rev() {
                    let mut sum = 0;
                    for k in i..=j {
                        sum += rowi[k] * matrix[k][j] % MODULO;
                    }
                    rowi[j] = sum % MODULO;
                }
            }
        }
        // matrix = matrix * matrix
        for i in 0..mat_len {
            for j in (i..mat_len).rev() {
                let mut sum = 0;
                for k in i..=j {
                    sum += matrix[i][k] * matrix[k][j] % MODULO;
                }
                matrix[i][j] = sum % MODULO;
            }
        }
        n >>= 1;
    }
    result
}

fn main() {
    input! {
        n: u64,
        m: u64,
    }
    let primes = [2, 3, 5, 7, 11, 13];
    let matrix_size = (1 << primes.len()) + 1;
    let prime_factors = (1..=m).map(|x| {
        let mut x = x;
        let mut factors = [0u32; 6];
        let mut bitmap = 0usize;
        for (i, &p) in primes.iter().enumerate() {
            while x % p == 0 {
                x /= p;
                factors[i] += 1;
                bitmap |= 1 << i;
            }
        }
        (bitmap, factors)
    }).collect::<Vec<_>>();
    let mut matrix = vec![vec![0u64; matrix_size]; matrix_size];
    for (i, rowi) in matrix[..matrix_size - 1].iter_mut().enumerate() {
        // 操作を行うが、数字を選択しない
        rowi[i] = m;
        for k in 1..=m {
            let (bitmap, factors) = prime_factors[k as usize - 1];
            let can_select = bitmap & !i;
            let mut subset = can_select;
            while subset > 0 {
                let mut count = 1;
                for b in 0..6 {
                    if subset & (1 << b) != 0 {
                        count *= factors[b] as u64;
                    }
                }
                rowi[i | subset] += count;
                subset = (subset - 1) & can_select;
            }
        }
    }
    matrix[matrix_size - 2][matrix_size - 1] = 1;
    matrix[matrix_size - 1][matrix_size - 1] = 1;
    let result = matrix_pow(&mut matrix, n + 1);
    // result.iter().for_each(|row| {
    //     eprintln!("{:?}", row)
    // });
    let ans = result[..matrix_size - 1].iter().map(|row| row.last().unwrap()).sum::<u64>() % MODULO;
    println!("{}", ans - 1);
}
