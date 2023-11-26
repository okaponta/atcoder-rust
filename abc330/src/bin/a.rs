use proconio::input;

fn main() {
    input! {
        n:usize,
        l:usize,
        a:[usize;n],
    }
    println!("{}", a.into_iter().filter(|a| &l <= a).count());
}
