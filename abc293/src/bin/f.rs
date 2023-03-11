use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t:usize,
        n:[usize;t],
    }
    for n in n {
        let mut ans = 0;
        for i in 2..=1000 {
            if is_ok(n, i) {
                ans += 1;
            }
        }
        for i in 2..64 {
            let mut lower = 1001;
            let mut upper = 1 << 60;
            while upper - lower > 1 {
                let med = (lower + upper) / 2;
                if n < calc(med, i) {
                    upper = med;
                } else {
                    lower = med;
                }
            }
            if n == calc(lower, i) {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}

fn is_ok(mut n: usize, b: usize) -> bool {
    let mut tmp = 1usize;
    while tmp.saturating_mul(b) <= n {
        tmp *= b;
    }
    while 0 < tmp {
        if tmp <= n {
            n -= tmp;
        }
        tmp /= b;
    }
    n == 0
}

fn calc(b: usize, i: usize) -> usize {
    let mut res: usize = 0;
    let mut tmp: usize = 1;
    for j in 0..7 {
        if i >> j & 1 == 1 {
            res = res.saturating_add(tmp);
        }
        tmp = tmp.saturating_mul(b);
    }
    res
}
