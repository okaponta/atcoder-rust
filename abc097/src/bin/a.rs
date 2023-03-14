use proconio::input;

fn main() {
    input! {
        a:i32,
        b:i32,
        c:i32,
        d:i32,
    }
    println!(
        "{}",
        if ((a - b).abs() <= d && (b - c).abs() <= d) || (a - c).abs() <= d {
            "Yes"
        } else {
            "No"
        }
    );
}
