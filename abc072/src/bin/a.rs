use proconio::input;

fn main() {
    input! {
        x:usize,
        t:usize,
    }
    println!("{}", x.saturating_sub(t));
}
