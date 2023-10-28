use proconio::input;

fn main() {
    input! {
        x:i32,
        y:i32,
    }
    println!(
        "{}",
        if y - x <= 2 && -3 <= y - x {
            "Yes"
        } else {
            "No"
        }
    );
}
