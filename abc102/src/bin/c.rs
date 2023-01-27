use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    let mut first = (0..n)
        .into_iter()
        .map(|i| a[i] - (i + 1) as i64)
        .collect::<Vec<_>>();
    first.sort();
    let b = first[n / 2];
    println!("{}", first.into_iter().fold(0, |s, ai| s + (ai - b).abs()));
}
