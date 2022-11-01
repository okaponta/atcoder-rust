use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;5],
    }
    a.sort();
    println!("{}", 4 + (n + a[0] - 1) / a[0]);
}
