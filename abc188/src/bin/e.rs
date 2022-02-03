use proconio::{input, marker::Usize1};

const INF: i64 = 1 << 60;

fn main() {
    input! {
        n:usize,m:usize,
        a:[i64;n],
        xy:[(Usize1,Usize1);m],
    }
    let mut dp = vec![INF; n];
    let mut path = vec![vec![]; n];
    for (x, y) in xy {
        path[x].push(y);
    }
    for i in 0..n {
        for &j in &path[i] {
            dp[j] = dp[j].min(dp[i]).min(a[i]);
        }
    }
    println!(
        "{}",
        (0..n).into_iter().map(|e| a[e] - dp[e]).max().unwrap()
    );
}
