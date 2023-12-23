use proconio::{fastout, input};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        mut r:[usize;n],
        x:[usize;q],
    }
    r.sort();
    let mut s = vec![0];
    for i in 0..n {
        s.push(s[i] + r[i]);
    }
    s.remove(0);
    for x in x {
        println!("{}", s.upper_bound(&x));
    }
}
