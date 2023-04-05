use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
        x:usize,
    }
    println!("{}", if a <= x && x <= a + b { "YES" } else { "NO" });
}
