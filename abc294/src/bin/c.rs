use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;n],
        b:[usize;m],
    }
    let mut c = vec![];
    for i in 0..n {
        c.push(a[i]);
    }
    for i in 0..m {
        c.push(b[i]);
    }
    let mut map = HashMap::new();
    c.sort();
    for i in 0..n + m {
        map.insert(c[i], i + 1);
    }
    let mut ansa = vec![];
    for i in 0..n {
        ansa.push(map.get(&a[i]).unwrap());
    }
    println!("{}", ansa.iter().join(" "));
    let mut ansb = vec![];
    for i in 0..m {
        ansb.push(map.get(&b[i]).unwrap());
    }
    println!("{}", ansb.iter().join(" "));
}
