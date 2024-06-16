use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:i64,
        t:[i64;n],
    }
    let mut bef = -1 << 60;
    for t in t {
        let ans = bef.max(t) + a;
        println!("{}", ans);
        bef = ans;
    }
}
