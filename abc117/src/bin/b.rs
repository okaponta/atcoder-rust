use proconio::input;

fn main() {
    input! {
        n:usize,
        mut l:[usize;n],
    }
    l.sort();
    let max = l.remove(n - 1);
    let sum = l.into_iter().sum::<usize>();
    println!("{}", if max < sum { "Yes" } else { "No" });
}
