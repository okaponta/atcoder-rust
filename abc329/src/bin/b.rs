use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    a.sort();
    a.dedup();
    a.pop();
    println!("{}", a.pop().unwrap());
}
