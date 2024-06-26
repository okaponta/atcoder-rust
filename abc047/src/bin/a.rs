use proconio::input;

fn main() {
    input! {
        mut a:[usize;3],
    }
    a.sort();
    println!("{}", if a[0] + a[1] == a[2] { "Yes" } else { "No" });
}
