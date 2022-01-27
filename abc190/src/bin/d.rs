use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        target:i64,
    }
    println!(
        "{}",
        divisor(2 * target)
            .iter()
            .filter(|&ni| ((2 * target / ni) + 1 - ni) % 2 == 0)
            .count()
    );
}

fn divisor(n: i64) -> Vec<i64> {
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
