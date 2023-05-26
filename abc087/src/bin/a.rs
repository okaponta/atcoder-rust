use proconio::input;

fn main() {
    input! {
        x:usize,
        a:usize,
        b:usize,
    }
    println!("{}", (x - a) % b);
}
