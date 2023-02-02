use proconio::input;

fn main() {
    input! {
        d:usize,
        mut n:usize,
    }
    if n == 100 {
        n += 1;
    }
    println!("{}", (0..d).into_iter().fold(n, |s, _| s * 100));
}
