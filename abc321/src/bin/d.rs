use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        m:usize,
        p:usize,
        a:[usize;n],
        mut b:[usize;m],
    }
    b.sort();
    let mut bs = vec![0];
    for i in 0..m {
        bs.push(bs[i] + b[i]);
    }
    let mut ans = 0;
    for a in a {
        let sat = b.upper_bound(&(p.saturating_sub(a)));
        ans += a * sat + bs[sat];
        ans += (m - sat) * p;
    }
    println!("{:?}", ans);
}
