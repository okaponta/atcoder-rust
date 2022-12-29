use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        t:usize,
        a:[usize;n],
    }
    let mut s = vec![0];
    for i in 0..n {
        s.push(s[i] + a[i]);
    }
    let rem = t % s[n];
    let i = s.lower_bound(&rem);
    println!("{} {}", i, rem - s[i - 1]);
}
