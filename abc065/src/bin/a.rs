use proconio::input;

fn main() {
    input! {
        x:i64,
        a:i64,
        b:i64,
    }
    println!(
        "{}",
        if b <= a {
            "delicious"
        } else if b <= a + x {
            "safe"
        } else {
            "dangerous"
        }
    );
}
