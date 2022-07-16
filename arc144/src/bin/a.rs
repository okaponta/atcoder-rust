use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    println!("{}", 2 * n);
    let mut ans = (n % 4).to_string();
    if n % 4 == 0 {
        ans = "".to_string();
    }
    for _ in 0..n / 4 {
        ans = ans + "4";
    }
    println!("{}", ans);
}
