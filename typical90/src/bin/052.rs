use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[[usize;6];n],
    }
    println!(
        "{}",
        a.into_iter().fold(1usize, |s, a| {
            (s * a.into_iter().sum::<usize>()) % 1_000_000_007
        })
    );
}
