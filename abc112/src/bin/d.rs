use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    println!(
        "{}",
        divisor(m)
            .into_iter()
            .filter(|&i| n <= m / i)
            .max()
            .unwrap()
    );
}

fn divisor(n: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut upper = vec![];
    for i in 1..=n.sqrt() {
        if n % i == 0 {
            res.push(i);
            if i != n / i {
                upper.push(n / i);
            }
        }
    }
    upper.reverse();
    res.append(&mut upper);
    res
}
