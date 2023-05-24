use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        d:usize,
        mut a:[usize;n],
        b:[usize;m],
    }
    a.sort();
    a.reverse();
    let set = b.into_iter().collect::<BTreeSet<_>>();
    for a in a {
        if let Some(b) = set.range(a.saturating_sub(d)..=a + d).last() {
            println!("{}", a + b);
            return;
        }
    }
    println!("-1");
}
