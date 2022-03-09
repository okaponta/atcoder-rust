use proconio::input;

fn main() {
    input! {n:usize}
    println!(
        "{}",
        (1..=n)
            .into_iter()
            .filter(|i| !(i % 3 == 0 || i % 5 == 0))
            .sum::<usize>()
    );
}
