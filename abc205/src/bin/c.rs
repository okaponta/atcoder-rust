use proconio::input;

fn main() {
    input! {
       mut a:i32,mut b:i32,c:i32,
    }
    if c % 2 == 0 {
        a = a.abs();
        b = b.abs();
    }
    println!(
        "{}",
        if a < b {
            "<"
        } else if a == b {
            "="
        } else {
            ">"
        }
    )
}
