use proconio::input;

fn main() {
    input! {
            n: i32,
            k: i32,
    }
    println!("{}", n * (n + 1) * k * 50 + k * (k + 1) * n / 2)
}
