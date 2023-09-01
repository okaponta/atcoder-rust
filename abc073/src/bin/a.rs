use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    println!(
        "{}",
        if n % 10 == 9 || n / 10 == 9 {
            "Yes"
        } else {
            "No"
        }
    );
}
