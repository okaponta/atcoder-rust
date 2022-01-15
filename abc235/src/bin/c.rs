use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
       n:usize,q:usize,
       a:[usize;n],
       xk:[(usize,usize);q],
    }
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        map.entry(a[i]).or_insert(vec![]).push(i + 1);
    }
    for (x, k) in xk {
        if let Some(val) = map.get(&x) {
            if val.len() >= k {
                println!("{}", val[k - 1]);
                continue;
            }
        }
        println!("{}", -1);
    }
}
