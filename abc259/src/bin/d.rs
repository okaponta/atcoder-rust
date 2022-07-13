use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n:usize,
        s:(i64,i64),
        t:(i64,i64),
        xyr:[(i64,i64,i64);n],
    }
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        for j in i..n {
            if is_cross_circumference(xyr[i].0, xyr[i].1, xyr[i].2, xyr[j].0, xyr[j].1, xyr[j].2) {
                uf.union(i, j);
            }
        }
    }
    let mut start = 0;
    let mut end = 0;
    for i in 0..n {
        let sd = dist(xyr[i].0, xyr[i].1, s.0, s.1);
        if sd == (xyr[i].2).pow(2) {
            start = i;
        }
        let td = dist(xyr[i].0, xyr[i].1, t.0, t.1);
        if td == (xyr[i].2).pow(2) {
            end = i;
        }
    }
    println!("{}", if uf.equiv(start, end) { "Yes" } else { "No" });
}

fn is_cross_circumference(x1: i64, y1: i64, r1: i64, x2: i64, y2: i64, r2: i64) -> bool {
    let d = dist(x1, y1, x2, y2);
    if (r1 + r2).pow(2) < d {
        return false;
    }
    if d < (r1 - r2).pow(2) {
        return false;
    }
    true
}

fn dist(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x1 - x2).pow(2) + (y1 - y2).pow(2)
}
