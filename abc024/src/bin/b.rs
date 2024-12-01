use proconio::*;

fn main() {
    input! {n:usize,t:usize,a:[usize;n]}
    let mut ans = 0;
    let mut start = 0;
    let mut end = 0;
    for a in a {
        if end < a {
            ans += end - start;
            start = a;
        }
        end = a + t;
    }
    ans += end - start;
    println!("{}", ans);
}
