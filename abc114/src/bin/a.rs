use proconio::input;

fn main() {
    input! {
        x:usize,
    }
    println!(
        "{}",
        if x == 3 || x == 5 || x == 7 {
            "YES"
        } else {
            "NO"
        }
    );
}
