use proconio::input;

// 2点間の距離の2乗
fn dist2(p1: &(f64, f64), p2: &(f64, f64)) -> f64 {
    let d0 = p1.0 - p2.0;
    let d1 = p1.1 - p2.1;
    d0 * d0 + d1 * d1
}

// (x, y)から各点への距離の最大値
fn f_xy(points: &Vec<(f64, f64)>, x: f64, y: f64) -> f64 {
    let mut res = 0.0;
    for p in points.iter() {
        let d = dist2(p, &(x, y));
        if d > res {
            res = d;
        }
    }
    res
}

// xを固定したときのf_xyの最小値
fn g_x(points: &Vec<(f64, f64)>, x: f64) -> f64 {
    let mut l = 0.0;
    let mut r = 1000.0;
    while r - l > 10e-9 {
        let m1 = (2.0 * l + r) / 3.0;
        let m2 = (l + 2.0 * r) / 3.0;
        let t1 = f_xy(points, x, m1);
        let t2 = f_xy(points, x, m2);
        if t1 < t2 {
            r = m2;
        } else {
            l = m1;
        }
    }
    f_xy(points, x, l)
}

/*
    (1)
    中心となる点を求めたい。
    中心となる点は、各点への距離の最大値が最小になる点である。

    (2)
    中心のx軸を三分探索する。
        x方向に左から右へ動かしていくとき、
        最もよいyを選んだ際の各点への距離の最大値は下に凸になるはず

    xが固定されたときの最もよいyは、これも三分探索で求まる。
*/

fn main() {
    input! {
        n:usize,
        xy:[(f64,f64);n],
    }

    let mut l = 0.0;
    let mut r = 1000.0;
    while r - l > 10e-9 {
        let m1 = (2.0 * l + r) / 3.0;
        let m2 = (l + 2.0 * r) / 3.0;
        let t1 = g_x(&xy, m1);
        let t2 = g_x(&xy, m2);
        if t1 < t2 {
            r = m2;
        } else {
            l = m1;
        }
    }
    let ans = (g_x(&xy, l)).sqrt();
    println!("{}", ans);
}
