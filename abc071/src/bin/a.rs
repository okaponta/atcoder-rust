use proconio::input;

fn main() {
    input! {
        x:i32,
        a:i32,
        b:i32,
    }
    if (x - a).abs() < (x - b).abs() {
        println!("A");
    } else {
        println!("B");
    }
}
