use proconio::input;

fn main() {
    input! {
        a:[usize;5],
    }
    let ans1 = a.iter().map(|i| (i + 9) / 10 * 10).sum::<usize>();
    let ans2 = a
        .iter()
        .map(|i| i % 10)
        .filter(|i| i != &0)
        .min()
        .unwrap_or(10);
    println!("{}", ans1 + ans2 - 10);
}
