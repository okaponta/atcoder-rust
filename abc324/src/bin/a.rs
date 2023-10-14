use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    a.sort();
    a.dedup();
    println!("{}", if a.len() == 1 { "Yes" } else { "No" });
}
