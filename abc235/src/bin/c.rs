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
        let count = map.entry(a[i]).or_insert(vec![]);
        count.push(i + 1);
    }
    for (x, k) in xk {
        if !map.contains_key(&x) {
            println!("{}", -1);
            continue;
        }
        let target = map.get(&x).unwrap();
        if target.len() < k {
            println!("{}", -1);
            continue;
        }
        println!("{}", target[k - 1]);
    }
}
