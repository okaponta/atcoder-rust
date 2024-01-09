use std::f64::consts::PI;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        xy:[(f64,f64);n],
    }
    let mut ans: f64 = 0.0;
    for i in 0..n {
        let angle = (0..n)
            .into_iter()
            .filter(|&j| i != j)
            .map(|j| (xy[j].1 - xy[i].1).atan2(xy[j].0 - xy[i].0))
            .sorted_by(|&a, &b| std(a).partial_cmp(&std(b)).unwrap())
            .collect::<Vec<f64>>();
        for j in 0..(n - 1) {
            let d = (angle[j] - angle[(j + 1) % (n - 1)]).abs();
            let tmp = if d > PI { PI * 2.0 - d } else { d };
            ans = ans.max(tmp);
        }
    }
    println!("{}", ans * 180.0 / PI);
}

fn std(a: f64) -> f64 {
    if a < 0.0 {
        a + PI
    } else {
        a
    }
}
