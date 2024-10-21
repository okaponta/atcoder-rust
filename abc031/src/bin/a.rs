use proconio::*;

fn main() {
    input! {a:usize,d:usize}
    println!("{}", (a.min(d) + 1) * a.max(d));
}
