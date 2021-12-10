use proconio::input;

fn main() {
    input! {
       n:i32,d:i64,
       xy:[(i64,i64);n],
    }
    println!(
        "{}",
        xy.iter().filter(|(x, y)| x * x + y * y <= d * d).count()
    );
}
