use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        mut uvbc:[(Usize1,Usize1,usize,usize);m],
    }
    uvbc.sort();
    let mut lower = 0.0;
    let mut upper = 1e5;
    while upper - lower > 1e-10 {
        let med = (lower + upper) / 2.0;
        if is_ok(med, &uvbc, n) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", lower);
}

fn is_ok(m: f64, uvbc: &Vec<(usize, usize, usize, usize)>, n: usize) -> bool {
    let mut dp = vec![-1e10; n];
    dp[0] = 0.0;
    for &(u, v, b, c) in uvbc {
        let tmp = dp[u] + b as f64 - c as f64 * m;
        if dp[v] < tmp {
            dp[v] = tmp;
        }
    }
    0.0 < dp[n - 1]
}
