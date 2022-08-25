use proconio::input;

fn main() {
    input! {
        x:i32,
        a:i32,
    }
    println!("{}", if x - a < 0 { 0 } else { 10 });
}
