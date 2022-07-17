use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,x:usize,y:usize,z:usize,
        a:[usize;n],
        b:[usize;n],
    }
    let mut math = vec![];
    let mut eng = vec![];
    let mut sum = vec![];
    for i in 0..n {
        math.push((a[i], i));
        eng.push((b[i], i));
        sum.push((a[i] + b[i], i));
    }
    math.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    eng.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    sum.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    let mut set = HashSet::new();
    let mut idx = 0;
    while set.len() < x {
        set.insert(math[idx].1);
        idx += 1;
    }
    idx = 0;
    while set.len() < x + y {
        set.insert(eng[idx].1);
        idx += 1;
    }
    idx = 0;
    while set.len() < x + y + z {
        set.insert(sum[idx].1);
        idx += 1;
    }
    let mut ans = vec![];
    for i in set {
        ans.push(i + 1);
    }
    ans.sort();
    for i in ans {
        println!("{}", i);
    }
}
