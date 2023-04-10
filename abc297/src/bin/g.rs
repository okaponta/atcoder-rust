use proconio::input;

fn main() {
    input! {
        n:usize,
        l:usize,
        r:usize,
        a:[usize;n],
    }
    let g = a.into_iter().fold(0, |g, a| g ^ ((a % (l + r)) / l));
    println!("{}", if g == 0 { "Second" } else { "First" });
}
