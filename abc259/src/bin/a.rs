use proconio::input;

fn main() {
    input! {
        _:usize,
        m:usize,
        x:usize,
        t:usize,
        d:usize,
    }
    println!("{}", (t + m * d - x * d).min(t));
}
