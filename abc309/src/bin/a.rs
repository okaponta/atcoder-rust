use proconio::input;

fn main() {
    input! {
        a:i32,
        b:i32,
    }
    println!(
        "{}",
        if b - a == 1 && a % 3 != 0 {
            "Yes"
        } else {
            "No"
        }
    )
}
