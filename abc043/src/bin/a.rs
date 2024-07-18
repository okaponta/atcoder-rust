use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    println!("{}", n * (n + 1) / 2);
}
