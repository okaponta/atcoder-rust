use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
        h:usize,
    }
    println!("{}", (a + b) * h / 2);
}
