use proconio::input;

fn main() {
    input! {
       x:i32,y:i32,
    }
    println!("{}", if x >= y { 0 } else { (y - x - 1) / 10 + 1 });
}
