use proconio::input;

fn main() {
    input! {
        d:usize,
    }
    println!(
        "{}",
        (0..25 - d)
            .into_iter()
            .fold("Christmas".to_string(), |a, _| a + " Eve")
    );
}
