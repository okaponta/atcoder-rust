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
            let d = dist(xyr[i].0, xyr[i].1, xyr[j].0, xyr[j].1);
            if is_inside(xyr[i].0, xyr[i].1, xyr[i].2, xyr[j].0, xyr[j].1) {
                if pow2(xyr[i].2 - xyr[j].2) <= d {
                    uf.union(i, j);
                }
            } else {
                if d <= pow2(xyr[i].2 + xyr[j].2) {
                    uf.union(i, j);
                }
            }
        }
    }
    let mut start = 0;
    for i in 0..n {
        let d = dist(xyr[i].0, xyr[i].1, s.0, s.1);
        if d == pow2(xyr[i].2) {
            start = i;
            break;
        }
    }
    let mut end = 0;
    for i in 0..n {
        let d = dist(xyr[i].0, xyr[i].1, t.0, t.1);
        if d == pow2(xyr[i].2) {
            end = i;
            break;
        }
    }
    println!("{}", if uf.equiv(start, end) { "Yes" } else { "No" });
}

fn dist(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    pow2(x1 - x2) + pow2(y1 - y2)
}

fn pow2(i: i64) -> i64 {
    i * i
}

fn is_inside(x1: i64, y1: i64, r: i64, x2: i64, y2: i64) -> bool {
    let d = dist(x1, y1, x2, y2);
    d < pow2(r)
}
