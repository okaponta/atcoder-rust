use proconio::input;

fn main() {
    input! {
       c:char,
    }
    println!("{}", if c.is_ascii_uppercase() { "A" } else { "a" });
}
