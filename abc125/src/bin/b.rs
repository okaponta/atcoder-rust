use proconio::input;

fn main() {
    input! {
        n:usize,
        v:[usize;n],
        c:[usize;n],
    }
    println!(
        "{}",
        v.iter()
            .zip(c.iter())
            .map(|(&v, &c)| v.saturating_sub(c))
            .sum::<usize>()
    )
}
