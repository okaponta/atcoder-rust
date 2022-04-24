use std::mem::swap;

use proconio::input;

fn main() {
    input! {
        t:usize,
    }
    for _ in 0..t {
        testcase();
    }
}

fn testcase() {
    input! {
        mut n:i64,
        mut a:i64,mut b:i64,
        x:i64,mut y:i64,mut z:i64,
    }
    // aの方がコスパよくする
    if y * b > z * a {
        swap(&mut a, &mut b);
        swap(&mut y, &mut z);
    }
    let mut ans = n * x;
    if n / a < a {
        // aの全探索
        for i in 0..=n / a {
            let ca = i * y;
            let rem = n - a * i;
            let tmp = ca + (rem / b) * z + (rem % b) * x;
            ans = ans.min(tmp);
        }
    } else {
        // bの全探索
        for i in 0..=a.min(n / b) {
            let cb = i * z;
            let rem = n - b * i;
            let tmp = cb + (rem / a) * y + (rem % a) * x;
            ans = ans.min(tmp);
        }
    }
    println!("{}", ans);
}
