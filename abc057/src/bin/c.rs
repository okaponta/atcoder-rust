use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut ans = 1 << 10;
    for a in divisor(n) {
        let b = n / a;
        let len = a.to_string().len().max(b.to_string().len());
        ans = ans.min(len);
    }
    println!("{}", ans);
}

fn divisor(n: usize) -> Vec<usize> {
    let mut res = vec![];
    for i in 1..=n.sqrt() {
        if n % i == 0 {
            res.push(i);
        }
    }
    res
}
