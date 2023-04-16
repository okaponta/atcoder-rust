use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize,
        d:usize,
    }
    println!("{}", a.min(b) + c.min(d));
}
