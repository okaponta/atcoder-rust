use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    println!(
        "{}",
        (1..10)
            .into_iter()
            .map(|i| i * 111)
            .filter(|i| n <= *i)
            .min()
            .unwrap()
    );
}
