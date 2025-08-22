use proconio::*;
fn main() {
    input! {n:usize, a:[usize;n]}
    for i in (0..=n).rev() {
        if i <= a.iter().filter(|&&a| i <= a).count() {
            println!("{i}");
            return;
        }
    }
}
