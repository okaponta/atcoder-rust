use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
    }
    println!("{}", if b / 2 == a { "Yes" } else { "No" });
}
