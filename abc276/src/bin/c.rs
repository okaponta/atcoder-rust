use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        mut p:[usize;n],
    }
    p.prev_permutation();
    println!("{}", p.iter().join(" "));
}
