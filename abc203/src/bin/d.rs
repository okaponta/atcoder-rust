use proconio::input;

fn main() {
    input! {
       n:usize,
       k:usize,
       a:[[i64;n];n],
    }
    let target = k * k / 2 + 1;
    let mut lower = -1;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if is_ok(med, n, k, target, &a) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", lower);
}

fn is_ok(m: i64, n: usize, k: usize, target: usize, a: &Vec<Vec<i64>>) -> bool {
    let mut csum = vec![vec![0; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            csum[i][j] = csum[i - 1][j] + csum[i][j - 1] - csum[i - 1][j - 1];
            if a[i - 1][j - 1] >= m {
                csum[i][j] += 1;
            }
        }
    }

    for i in 0..=n - k {
        for j in 0..=n - k {
            let cnt = csum[i + k][j + k] - csum[i + k][j] - csum[i][j + k] + csum[i][j];
            if cnt < target {
                return false;
            }
        }
    }
    true
}
