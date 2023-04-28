use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
    }
    println!(
        "{}",
        (a..=b)
            .into_iter()
            .filter(|i| i / 10000 == i % 10 && i % 100 / 10 == i % 10000 / 1000)
            .count()
    );
}
