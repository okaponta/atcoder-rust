use std::collections::HashMap;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        t:usize,
        ab:[(Usize1,usize);t],
    }
    let mut points = vec![0; n];
    let mut map = HashMap::new();
    map.insert(0, n);
    for (a, b) in ab {
        let bef = points[a];
        *map.entry(bef).or_insert(0) -= 1;
        if map[&bef] == 0 {
            map.remove(&bef);
        }
        *map.entry(bef + b).or_insert(0) += 1;
        points[a] += b;
        println!("{}", map.len());
    }
}
