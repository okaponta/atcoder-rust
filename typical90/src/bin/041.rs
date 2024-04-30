use proconio::input;

fn main() {
    input! {
        n: usize,
        mut p: [(i64, i64); n]
    };
    let t = sort_point_clockwise(n, p);
    let mut ep = t.len() as i64;
    let s = convex_area(&t);
    for i in 0..t.len() {
        let (ax, ay) = t[i];
        let (bx, by) = t[(i + 1) % t.len()]; // 一周を考慮
        let (vx, vy) = ((ax - bx).abs(), (ay - by).abs());
        let r = gcd(vx, vy);
        ep += r - 1;
    }

    // ピックの定理で格子点の数を求める
    let mut ans = (s - ep + 2) / 2;
    // 周囲の点を加算(但し、端店を除く)
    ans += ep - n as i64;
    println!("{}", ans);
}

// 点を時計回りにソートする
fn sort_point_clockwise(n: usize, mut p: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    p.sort();
    // 右回り
    let mut g1 = vec![p[0], p[1]];
    // 左回り
    let mut g2 = vec![p[0], p[1]];
    for i in 2..n {
        while g1.len() >= 2 && outer_product_p(g1[g1.len() - 2], g1[g1.len() - 1], p[i]) <= 0 {
            g1.pop();
        }
        while g2.len() >= 2 && outer_product_p(g2[g2.len() - 2], g2[g2.len() - 1], p[i]) >= 0 {
            g2.pop();
        }
        g1.push(p[i]);
        g2.push(p[i]);
    }

    // 一周
    let mut t = g1;
    t.append(&mut g2.into_iter().skip(1).rev().skip(1).collect());
    t
}

// 上記の点だけ渡す版
// P1P2とP1P3の外積を求める。外積なので三角形の二倍
fn outer_product_p(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64)) -> i64 {
    let a = p2.0 - p1.0;
    let b = p2.1 - p1.1;
    let c = p3.0 - p1.0;
    let d = p3.1 - p1.1;
    a * d - b * c
}

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

// 凸多角形の面積の2倍を求める
// 三角形に分割して計算を行う
fn convex_area(xy: &Vec<(i64, i64)>) -> i64 {
    let mut res = 0;
    let n = xy.len();
    for i in 2..n {
        res += outer_product_p(xy[0], xy[i - 1], xy[i]).abs();
    }
    res
}
