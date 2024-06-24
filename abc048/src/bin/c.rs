use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize,
        mut a:[usize;n],
    }
    let mut ans = 0;
    if x < a[0] {
        ans += a[0] - x;
        a[0] = x;
    }
    for i in 1..n {
        if x < a[i - 1] + a[i] {
            let t = a[i - 1] + a[i] - x;
            ans += t;
            a[i] -= t;
        }
    }
    println!("{}", ans);
}
