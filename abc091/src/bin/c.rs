use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        ab:[(usize,usize);n],
        cd:[(usize,usize);n],
    }
    let mut points = vec![];
    for (a, b) in ab {
        points.push((a, b, 0));
    }
    for (c, d) in cd {
        points.push((c, d, 1));
    }
    points.sort();
    let mut count = 0;
    let mut red = BTreeSet::new();
    for (_, y, c) in points {
        if c == 0 {
            red.insert(y);
        } else if let Some(&val) = red.range(..y).last() {
            red.remove(&val);
            count += 1;
        }
    }
    println!("{}", count);
}
