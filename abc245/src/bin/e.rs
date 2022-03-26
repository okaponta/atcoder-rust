use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n:usize,m:usize,
        a:[usize;n],
        b:[usize;n],
        c:[usize;m],
        d:[usize;m],
    }
    let mut abcd = vec![];
    for i in 0..n {
        abcd.push((a[i], b[i], 0));
    }
    for i in 0..m {
        abcd.push((c[i], d[i], 1));
    }
    abcd.sort_by(|a, b| match b.0.cmp(&a.0) {
        std::cmp::Ordering::Equal => match b.1.cmp(&a.1) {
            std::cmp::Ordering::Equal => b.2.cmp(&a.2),
            other => other,
        },
        other => other,
    });
    let mut set = BTreeSet::new();
    for (i, (_, y, is_choco)) in abcd.iter().enumerate() {
        if *is_choco != 0 {
            set.insert((*y, i));
        } else {
            let bo = set.range((*y, 0)..).next().map_or((0, 0), |i| *i);
            if bo.0 == 0 {
                println!("No");
                return;
            } else {
                set.remove(&bo);
            }
        }
    }
    println!("Yes");
}
