use proconio::input;

fn main() {
    input! {
        a:i32,b:i32,
    }
    println!("{}", if a > 9 || b > 9 { -1 } else { a * b });
}
