use proconio::*;

fn main() {
    input! {
        mut n:usize,m:usize,mut ab:[(usize,usize);m]
    }
    ab.sort_by(|a, b| (a.0 - a.1).cmp(&(b.0 - b.1)));
    let mut ans = 0;
    for (a, b) in ab {
        if n < a {
            continue;
        }
        let t = (n - a) / (a - b) + 1;
        ans += t;
        n -= t * (a - b);
    }
    println!("{}", ans);
}
