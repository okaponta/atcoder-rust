use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [u64; n],
        x: [u64; q],
    }
    a.sort();
    for x in x {
        println!("{}", n - a.lower_bound(&x));
    }
}
