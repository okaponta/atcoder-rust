use proconio::input;

fn main() {
    input! {
        a:[usize;3],
    }
    let b = a[0] * 100 + a[1] * 10 + a[2];
    println!("{}", if b % 4 == 0 { "YES" } else { "NO" });
}
