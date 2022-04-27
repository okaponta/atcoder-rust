use proconio::input;

fn main() {
    input! {
        ab:[i32;2],
    }
    println!(
        "{}",
        (1..4)
            .into_iter()
            .filter(|i| !ab.contains(i))
            .next()
            .unwrap()
    );
}
