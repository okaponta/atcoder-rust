use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize,
    }
    println!("{}", c.min(b / a));
}
