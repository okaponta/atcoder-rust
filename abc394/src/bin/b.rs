use proconio::*;

fn main() {
    input! {
        n:usize,
        mut s:[String;n],
    }
    s.sort_by_key(|s| s.len());
    s.iter().for_each(|s| print!("{}", s));
    println!();
}
