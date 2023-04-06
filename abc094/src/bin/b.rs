use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        _:usize,
        m:usize,
        x:usize,
        a:[usize;m],
    }
    let l = a.lower_bound(&x);
    println!("{}", (m - l).min(l));
}
