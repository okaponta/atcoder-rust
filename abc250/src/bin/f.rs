use proconio::input;

fn main() {
    input! {
        n:usize,
        xy:[(i64,i64);n],
    }
    let mut p = vec![];
    for i in 0..n * 2 {
        p.push(xy[i % n]);
    }
    // aの8倍
    let mut a_8 = 0;
    for i in 2..n {
        a_8 += outer_product(xy[0], xy[i - 1], xy[i]).abs();
    }

    let mut ans = a_8;
    let mut left = 0;
    let mut right = 1;
    let mut s_2 = 0;
    while left < n {
        if s_2 * 4 < a_8 {
            right += 1;
            s_2 += outer_product(p[left], p[right - 1], p[right]).abs();
        } else {
            s_2 -= outer_product(p[left], p[left + 1], p[right]).abs();
            left += 1;
        }
        ans = ans.min((a_8 - s_2 * 4).abs());
    }
    println!("{}", ans);
}

// P1P2とP1P3の外積を求める。外積なので三角形の二倍
fn outer_product(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64)) -> i64 {
    let a = p2.0 - p1.0;
    let b = p2.1 - p1.1;
    let c = p3.0 - p1.0;
    let d = p3.1 - p1.1;
    a * d - b * c
}
