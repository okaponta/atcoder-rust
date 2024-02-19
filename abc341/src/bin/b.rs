use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
        st:[(usize,usize);n-1],
    }
    for i in 0..n - 1 {
        a[i + 1] += a[i] / st[i].0 * st[i].1;
    }
    println!("{}", a[n - 1]);
}
