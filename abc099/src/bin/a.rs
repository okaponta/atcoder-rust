use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    println!("AB{}", if n < 1000 { "C" } else { "D" });
}
