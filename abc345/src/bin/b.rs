use proconio::input;

fn main() {
    input! {
        x:i64,
    }
    if 0 <= x {
        println!("{}", (x + 9) / 10);
    } else if -10 < x {
        println!("{}", 0);
    } else {
        println!("-{}", -x / 10);
    }
}
