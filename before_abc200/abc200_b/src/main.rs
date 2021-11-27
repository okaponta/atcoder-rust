use proconio::input;

fn main() {
    input! {
        mut n: i64,
        k: i64
    }
    for _ in 0..k {
        if n % 200 == 0 {
            n /= 200;
        } else {
            n = n * 1000 + 200;
        }
    }
    println!("{}", n)
}
