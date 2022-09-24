use proconio::input;

fn main() {
    input! {
        mut x:i64,
        mut y:i64,
        mut z:i64,
    }
    if x < 0 {
        x = -x;
        y = -y;
        z = -z;
    }
    let ans;
    if 0 < y && y < x {
        // ハンマーが必要
        if 0 < z && z < y {
            ans = x.abs();
        } else if z < 0 {
            ans = 2 * z.abs() + x.abs();
        } else {
            println!("-1");
            return;
        }
    } else {
        ans = x.abs();
    }
    println!("{}", ans);
}
