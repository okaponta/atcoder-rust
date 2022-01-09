use proconio::input;

fn main() {
    input! {
       a:i64,b:i64,_c:i64,k:i64,
    }
    println!(
        "{}",
        if k < a {
            k
        } else if k < a + b {
            a
        } else {
            2 * a + b - k
        }
    );
}
