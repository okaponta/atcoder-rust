use proconio::input;

fn main() {
    input! {
        x:i128,
    }
    println!("{}", if x > 0 { x / 10 } else { (x - 9) / 10 });
}
