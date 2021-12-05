use proconio::input;

fn main() {
    input! {
       n:i32,
       ab:[(i64,i64);n]
    }
    println!(
        "{}",
        ab.iter()
            .map(|(a, b)| ((a + b) * (b - a + 1) / 2))
            .sum::<i64>()
    );
}
