use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    let mut keys = vec![];
    for _ in 0..m {
        input! {
            a:usize,
            b:usize,
            c:[Usize1;b],
        }
        let mut target = 0;
        for c in c {
            target += 1 << c;
        }
        keys.push((a, target));
    }
    let mut dp = vec![1 << 60; 1 << n];
    dp[0] = 0;
    for i in 0..1 << n {
        for j in 0..m {
            dp[i | keys[j].1] = dp[i | keys[j].1].min(dp[i] + keys[j].0);
        }
    }
    if dp[(1 << n) - 1] == 1 << 60 {
        println!("-1");
    } else {
        println!("{}", dp[(1 << n) - 1]);
    }
}
