use proconio::input;

fn main() {
    input! {
        n:usize,
        a:usize,
    }
    println!("{}", n * n - a);
}
