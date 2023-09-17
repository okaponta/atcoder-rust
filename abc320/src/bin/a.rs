use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
    }
    println!(
        "{}",
        (0..a).fold(1, |s, _| s * b) + (0..b).fold(1, |s, _| s * a)
    );
}
