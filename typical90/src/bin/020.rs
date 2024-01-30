use proconio::input;

fn main() {
    input! {
        a:usize,
        b:u32,
        c:usize,
    }
    println!("{}", if a < c.pow(b) { "Yes" } else { "No" });
}
