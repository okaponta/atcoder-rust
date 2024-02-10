use proconio::input;

fn main() {
    input! {
        mut x:i64,
        mut y:i64,
    }
    if x == 0 || y == 0 {
        fn f(i: i64, inv: bool) {
            if 2 < i {
                println!("-1");
                return;
            }
            let mut ans = vec![2 / i, 0];
            if inv {
                ans.reverse();
            }
            println!("{} {}", ans[0], ans[1]);
        }
        if y == 0 {
            f(x, true);
        } else {
            f(y, false);
        }
        return;
    }
    let g = gcd(x.abs(), y.abs());
    if 2 < g {
        println!("-1");
        return;
    }
    let mut a = 0;
    let mut b = 0;
    extend_euclid(y, -x, &mut a, &mut b);
    if g == 1 {
        println!("{} {}", 2 * a, 2 * b);
    } else {
        println!("{} {}", a, b);
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn extend_euclid(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    if b == 0 {
        *x = 1;
        *y = 0;
        return a;
    }
    let d = extend_euclid(b, a % b, y, x);
    *y -= a / b * *x;
    d
}
