use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut ans = 1 << 60;
    for i in 1..=n.sqrt() {
        if n % i == 0 {
            ans = ans.min(i + n / i - 2);
        }
    }
    println!("{}", ans);
}
