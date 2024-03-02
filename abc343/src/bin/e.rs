use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        v1:i64,
        v2:i64,
        v3:i64,
    }
    if v1 + v2 * 2 + v3 * 3 != 1029 {
        println!("No");
        return;
    }
    for (i, j, k) in iproduct!(-7..8, -7..8, -7..8) {
        for (l, m, n) in iproduct!(-7..8, -7..8, -7..8) {
            let v = calc((0, 0, 0), (i, j, k), (l, m, n));
            if v.0 == v1 && v.1 == v2 && v.2 == v3 {
                println!("Yes");
                println!("{} {} {} {} {} {} {} {} {}", 0, 0, 0, i, j, k, l, m, n);
                return;
            }
        }
    }
    println!("No");
}

fn calc(a: (i64, i64, i64), b: (i64, i64, i64), c: (i64, i64, i64)) -> (i64, i64, i64) {
    let ab = c1(a, b);
    let bc = c1(b, c);
    let ca = c1(c, a);
    let v3 = c3(a, b, c);
    let v2 = bc + ca + ab - v3 * 3;
    let v1 = 1029 - v2 * 2 - v3 * 3;
    (v1, v2, v3)
}

fn c1(a: (i64, i64, i64), b: (i64, i64, i64)) -> i64 {
    c2(a.0, b.0) * c2(a.1, b.1) * c2(a.2, b.2)
}

fn c2(a: i64, b: i64) -> i64 {
    (a.min(b) + 7 - a.max(b)).max(0)
}

fn c3(a: (i64, i64, i64), b: (i64, i64, i64), c: (i64, i64, i64)) -> i64 {
    c4(a.0, b.0, c.0) * c4(a.1, b.1, c.1) * c4(a.2, b.2, c.2)
}

fn c4(a: i64, b: i64, c: i64) -> i64 {
    (a.min(b).min(c) + 7 - a.max(b).max(c)).max(0)
}
