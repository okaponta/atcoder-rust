use proconio::input;

fn main() {
    input! {
        n:usize,
        h:[usize;n],
    }
    let mut ans = 0;
    let mut max = 0;
    for h in h {
        max = max.max(h);
        if max <= h {
            ans += 1;
        }
    }
    println!("{}", ans);
}
