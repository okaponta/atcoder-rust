use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        mut a:[usize;n],
    }
    a.sort();
    let mut ans = 0;
    for i in 0..n {
        if a[i] == ans && ans < k {
            ans += 1;
        }
    }
    println!("{}", ans);
}
