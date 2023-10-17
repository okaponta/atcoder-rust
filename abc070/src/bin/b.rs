use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize,
        d:usize,
    }
    println!("{}", b.min(d).saturating_sub(a.max(c)));
}
