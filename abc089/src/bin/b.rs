use proconio::input;

fn main() {
    input! {
        n:usize,
        s:[char;n],
    }
    println!(
        "{}",
        if s.into_iter().any(|c| c == 'Y') {
            "Four"
        } else {
            "Three"
        }
    );
}
