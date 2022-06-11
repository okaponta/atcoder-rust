use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[Usize1;k],
        xy:[(i64,i64);n],
    }
    let aset = a.iter().collect::<HashSet<&usize>>();
    let mut dark = vec![];
    let mut light = vec![];
    for i in 0..n {
        if aset.contains(&&i) {
            light.push(xy[i]);
        } else {
            dark.push(xy[i]);
        }
    }
    let mut ans = 0;
    for (x, y) in dark {
        let mut min = 1 << 60;
        for (xx, yy) in &light {
            let d = (x - xx) * (x - xx) + (y - yy) * (y - yy);
            min = min.min(d);
        }
        ans = ans.max(min);
    }
    println!("{}", (ans as f64).sqrt());
}
