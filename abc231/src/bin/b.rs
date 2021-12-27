use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
       n:usize,
       s:[String;n],
    }
    let mut map = HashMap::new();
    for si in s {
        *map.entry(si).or_insert(0) += 1;
    }
    println!("{}", map.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap().0);
}
