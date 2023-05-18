use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    a.sort();
    a.reverse();
    let mut ans = 0;
    for i in 0..n {
        if i % 2 == 0 {
            ans += a[i];
        } else {
            ans -= a[i];
        }
    }
    println!("{}", ans);
}
