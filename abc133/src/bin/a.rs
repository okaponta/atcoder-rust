use proconio::input;

fn main() {
    input! {
        n:usize,
        a:usize,
        b:usize,
    }
    println!("{}", b.min(n * a));
}
