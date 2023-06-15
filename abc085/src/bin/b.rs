use proconio::input;

fn main() {
    input! {
        n:usize,
        mut d:[usize;n],
    }
    d.sort();
    d.dedup();
    println!("{}", d.len());
}
