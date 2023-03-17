use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n:usize,
        mut xy:[(i64,i64);n],
    }
    xy.sort();
    let mut lower = 0;
    let mut upper = 1 << 30;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if is_over(med, &xy) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", lower);
}

fn is_over(k: i64, xy: &Vec<(i64, i64)>) -> bool {
    let mut min = 1 << 30;
    let mut max = 0;
    let mut q = VecDeque::new();
    for &(x, y) in xy {
        while let Some((px, py)) = q.pop_front() {
            if x < px + k {
                q.push_front((px, py));
                break;
            }
            min = min.min(py);
            max = max.max(py);
        }
        if min + k <= y || y <= max - k {
            return true;
        }
        q.push_back((x, y));
    }
    false
}
