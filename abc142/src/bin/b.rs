use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        h:[usize;n],
    }
    println!("{}", h.iter().filter(|h| h >= &&k).count());
}
