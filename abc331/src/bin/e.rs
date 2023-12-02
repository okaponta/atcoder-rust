use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        l:usize,
        a:[usize;n],
        b:[usize;m],
        cd:[(Usize1,Usize1);l],
    }
    let mut bb = vec![];
    for i in 0..m {
        bb.push((b[i], i));
    }
    let mut set = HashSet::new();
    for (c, d) in cd {
        set.insert((c, d));
    }
    bb.sort();
    bb.reverse();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if set.contains(&(i, bb[j].1)) {
                continue;
            }
            ans = ans.max(a[i] + bb[j].0);
            break;
        }
    }
    println!("{}", ans);
}
