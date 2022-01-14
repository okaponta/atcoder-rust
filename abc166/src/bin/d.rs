use proconio::input;

fn main() {
    input! {
       x:i64,
    }
    for a in 0..120 {
        for b in -120..120 {
            if a * a * a * a * a - b * b * b * b * b == x {
                println!("{} {}", a, b);
                return;
            }
        }
    }
}
