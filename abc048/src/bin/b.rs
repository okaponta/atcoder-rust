use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
        x:usize,
    }
    println!("{}", 1 + b / x - (a + x - 1) / x);
}
