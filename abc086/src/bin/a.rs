use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
    }
    println!("{}", if a * b & 1 == 1 { "Odd" } else { "Even" });
}
