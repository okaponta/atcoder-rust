use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    let mut g = vec![0; n];
    let mut ans = vec![];
    let mut prev = 0;
    for i in 0..n {
        if a[i] < 0 {
            prev = i;
            ans.push(i + 1);
        } else {
            g[(a[i] - 1) as usize] = i;
        }
    }
    for _ in 1..n {
        ans.push(g[prev] + 1);
        prev = g[prev];
    }
    println!("{}", ans.iter().join(" "));
}
