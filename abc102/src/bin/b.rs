use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n]
    }
    a.sort();
    println!("{}", a[n - 1] - a[0]);
}
