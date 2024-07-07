use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        mut a:[usize;n],
    }
    a.sort();
    let mut ans = a[n - 1] - a[0];
    for i in 0..=k {
        if a[i + n - 1 - k] - a[i] < ans {
            ans = a[i + n - 1 - k] - a[i];
        }
    }
    println!("{}", ans);
}
