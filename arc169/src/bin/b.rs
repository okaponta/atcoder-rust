use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        s:usize,
        a:[usize;n],
    }
    let ss = ruiseki(&a);
    let mut p = vec![n; n + 1];
    for i in 0..n {
        p[i] = ss.lower_bound(&(ss[i] + s + 1)) - 1;
    }
    let mut ans = 0;
    let mut dp = vec![0; n + 2];
    for i in (0..n).rev() {
        dp[i] = dp[p[i]] + n - i;
        ans += dp[i];
    }
    println!("{}", ans);
}

fn ruiseki(a: &Vec<usize>) -> Vec<usize> {
    let mut res = vec![0];
    for i in 0..a.len() {
        res.push(res[i] + a[i]);
    }
    res
}
