use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
    }
    println!("{}", if b < a { a - 1 } else { a });
}
