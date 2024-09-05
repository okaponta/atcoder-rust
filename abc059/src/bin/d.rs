use proconio::input;

fn main() {
    input! {x:i64,y:i64}
    println!("{}", if (x - y).abs() < 2 { "Brown" } else { "Alice" });
}
