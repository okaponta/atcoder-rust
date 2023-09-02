use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        p:usize,
    }
    println!("{}", (n + p - m) / p);
}
