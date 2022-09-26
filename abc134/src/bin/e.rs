use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut set = BTreeSet::new();
    for i in 0..n {
        if let Some(&num) = set.range(..(a[i], 0)).last() {
            set.remove(&num);
        }
        set.insert((a[i], i));
    }
    println!("{}", set.len());
}
