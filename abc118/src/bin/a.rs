use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
    }
    println!("{}", if b % a == 0 { a + b } else { b - a });
}
