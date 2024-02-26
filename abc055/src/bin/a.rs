use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    println!("{}", n * 800 - n / 15 * 200);
}
