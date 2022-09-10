use proconio::input;

fn main() {
    input! {
        mut a:[usize;5],
    }
    a.sort();
    a.dedup();
    println!("{}", a.len());
}
