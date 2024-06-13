use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[i64;n],
    }
    if n % 2 == 0 {
        a.push(-1);
        a.push(-1);
    } else {
        a.push(-2);
        a.push(0);
    }
    a.sort();
    let mut ans = 1;
    for i in 0..n {
        if i % 2 != 0 {
            ans *= 2;
            ans %= 1_000_000_007;
        }
        if a[i + 2] - a[i] != 2 {
            println!("0");
            return;
        }
    }
    println!("{}", ans);
}
