use proconio::{fastout, input};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        mut a:[i64;n],
        x:[i64;q],
    }
    a.sort();
    let mut s = vec![a[0]];
    for i in 1..n {
        s.push(s[i - 1] + a[i]);
    }
    for xi in x {
        if n == 1 {
            println!("{}", (xi - a[0]).abs());
            continue;
        }
        if xi <= a[0] {
            println!("{}", s[n - 1] - xi * (n as i64));
            continue;
        }
        if a[n - 1] <= xi {
            println!("{}", xi * (n as i64) - s[n - 1]);
            continue;
        }
        let below = a.lower_bound(&xi);
        let over = a.upper_bound(&xi);
        let mut ans = 0;
        ans += xi * (below as i64) - s[below - 1];
        ans += s[n - 1] - s[over - 1] - xi * ((n - over) as i64);
        println!("{}", ans);
    }
}
