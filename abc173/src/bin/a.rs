use proconio::input;

fn main() {
    input! {
       n:i32,
    }
    println!("{}", if n % 1000 == 0 { 0 } else { 1000 - n % 1000 });
}
