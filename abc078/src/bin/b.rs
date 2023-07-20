use proconio::input;

fn main() {
    input! {
        x:usize,
        y:usize,
        z:usize,
    }
    println!("{}", (x - z) / (y + z));
}
