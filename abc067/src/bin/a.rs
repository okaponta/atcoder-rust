use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
    }
    println!(
        "{}",
        if a % 3 == 0 || b % 3 == 0 || (a + b) % 3 == 0 {
            "Possible"
        } else {
            "Impossible"
        }
    );
}
