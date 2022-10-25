use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
    }
    let ans = if a == b { a + b } else { a.max(b) * 2 - 1 };
    println!("{}", ans);
}
