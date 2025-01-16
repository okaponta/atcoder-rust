use proconio::*;

fn main() {
    input! {
        a:(i64,i64),
        b:(i64,i64),
        n:usize,
        xy:[(i64,i64);n],
    }
    println!(
        "{}",
        1 + (0..n)
            .into_iter()
            .filter(|&i| g(a, b, xy[i], xy[(i + 1) % n]))
            .count()
            / 2
    );
}

fn g(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64), p4: (i64, i64)) -> bool {
    f(p1, p2, p3) * f(p1, p2, p4) < 0 && f(p3, p4, p1) * f(p3, p4, p2) < 0
}

fn f(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64)) -> i64 {
    let a = p2.0 - p1.0;
    let b = p2.1 - p1.1;
    let c = p3.0 - p1.0;
    let d = p3.1 - p1.1;
    a * d - b * c
}
