use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:Chars,
        k:usize,
    }
    let first = n[0].to_digit(10).unwrap() as usize;
    let mut dp = vec![1, first - 1, 0, 0, 0];
    let mut sub = vec![0, 1, 0, 0, 0];
    for i in 1..n.len() {
        let mut next = dp.clone();
        let mut next_sub = vec![0; 5];
        let num = n[i].to_digit(10).unwrap() as usize;
        for j in 0..4 {
            if num == 0 {
                next_sub[j] = sub[j];
            } else {
                next_sub[j + 1] = sub[j];
                next[j + 1] += sub[j] * (num - 1);
                next[j] += sub[j];
            }
            next[j + 1] += dp[j] * 9;
        }
        dp = next;
        sub = next_sub;
    }
    println!("{}", dp[k] + sub[k]);
}
