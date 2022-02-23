use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let ans = a
        .iter()
        .filter(|i| *i % 2 == 0)
        .all(|i| i % 3 == 0 || i % 5 == 0);
    println!("{}", if ans { "APPROVED" } else { "DENIED" });
}
