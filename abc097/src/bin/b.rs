use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        x:usize,
    }
    let mut ans = 1;
    for b in 2..=x.sqrt() {
        let mut tmp = b * b;
        while tmp <= x {
            ans = ans.max(tmp);
            tmp *= b;
        }
    }
    println!("{}", ans);
}
