use proconio::input;

fn main() {
    input! {
       n:i32,
    }
    println!("AGC{:<03}", if n > 41 { n + 1 } else { n });
}
