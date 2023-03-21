use proconio::{input, marker::Usize1};

fn main() {
    input! {
       l:Usize1,
       r:usize,
    }
    let fact_num = eratosthenes(r);
    let mut ans = 0;
    for i in 2..=r {
        if fact_num[i] < 0 {
            continue;
        }
        let mut div = nc2(r / i - l / i) as i64;
        if fact_num[i] % 2 == 0 {
            div = -div;
        }
        ans += div;
    }
    for i in 2.max(l + 1)..=r {
        ans -= (r / i) as i64 - 1;
    }
    println!("{}", ans * 2);
}

fn nc2(n: usize) -> usize {
    n * (n - 1) / 2
}

fn eratosthenes(n: usize) -> Vec<i64> {
    let mut res = vec![0; n + 1];
    for i in 2..=n {
        if res[i] != 0 {
            continue;
        }
        for j in (i..=n).step_by(i) {
            res[j] += 1;
        }
        for j in (i * i..=n).step_by(i * i) {
            // iのときにカウント済なのでとばす
            res[j] = -100000000;
        }
    }
    res
}
