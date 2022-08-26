use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n:usize,x:usize,
        l:[usize;n],
    }
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + l[i];
    }
    println!("{}", s.upper_bound(&x));
}
