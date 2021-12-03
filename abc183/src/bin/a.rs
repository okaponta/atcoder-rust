use proconio::input;

fn main() {
    input! {
       x:i32,
    }
    println!("{}", if x < 0 { 0 } else { x });
}
