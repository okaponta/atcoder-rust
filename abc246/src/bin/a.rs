use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        xy:[(i32,i32);3],
    }
    let mut xset = HashSet::new();
    let mut yset = HashSet::new();
    for (x, y) in xy {
        if xset.contains(&x) {
            xset.remove(&x);
        } else {
            xset.insert(x);
        }
        if yset.contains(&y) {
            yset.remove(&y);
        } else {
            yset.insert(y);
        }
    }
    let x = xset.iter().next().unwrap();
    let y = yset.iter().next().unwrap();
    println!("{} {}", x, y);
}
