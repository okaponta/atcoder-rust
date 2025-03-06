use proconio::*;

fn main() {
    input! {
        n:usize,
        x:usize,
        ud:[(usize, usize);n],
    }
    let mut sum = 0;
    let mut l = 0;
    let mut r = 1 << 60;
    for i in 0..n {
        let s = ud[i].0 + ud[i].1;
        sum += s;
        r = r.min(s + 1);
    }
    while 1 < r - l {
        let mid = (l + r) / 2;
        if is_ok(mid, &ud, x, n) {
            l = mid;
        } else {
            r = mid;
        }
    }
    println!("{}", sum - l * n);
}

fn is_ok(h: usize, ud: &Vec<(usize, usize)>, x: usize, n: usize) -> bool {
    let mut min = 0usize;
    let mut max = h;
    for i in 0..n {
        let (u, d) = ud[i];
        min = min.saturating_sub(x).max(h.saturating_sub(u));
        max = (max + x).min(h).min(d);
        if max < min {
            return false;
        }
    }
    true
}
