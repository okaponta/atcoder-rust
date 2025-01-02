use proconio::*;

fn main() {
    input! {
        n:usize,
        xh:[(i64,i64);n],
    }
    let mut ans: f64 = -1.0;
    for i in 1..n {
        let (x1, h1) = xh[i - 1];
        let (x2, h2) = xh[i];
        ans = ans.max((h1 * (x2 - x1) - (h2 - h1) * x1) as f64 / (x2 - x1) as f64);
    }
    if ans < 0.0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
