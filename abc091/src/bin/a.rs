use proconio::input;

fn main() {
    input! {
        a:[usize;3],
    }
    println!("{}", if a[2] <= a[0] + a[1] { "Yes" } else { "No" });
}
