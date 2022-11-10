use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        x:i64,
        y:i64,
        a:[i64;n],
    }
    let mut xset = HashSet::new();
    let mut yset = HashSet::new();
    xset.insert(a[0]);
    yset.insert(0);
    for i in 1..n {
        let mut new_set = HashSet::new();
        if i % 2 == 0 {
            for x in xset {
                if x + a[i] <= 10000 {
                    new_set.insert(x + a[i]);
                }
                if -10000 <= x - a[i] {
                    new_set.insert(x - a[i]);
                }
            }
            xset = new_set;
        } else {
            for y in yset {
                if y + a[i] <= 10000 {
                    new_set.insert(y + a[i]);
                }
                if -10000 <= y - a[i] {
                    new_set.insert(y - a[i]);
                }
            }
            yset = new_set;
        }
    }
    println!(
        "{}",
        if xset.contains(&x) && yset.contains(&y) {
            "Yes"
        } else {
            "No"
        }
    );
}
