use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
        t:usize,
    }
    println!("{}", t / a * b);
}
