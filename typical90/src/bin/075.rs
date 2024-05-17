use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    println!(
        "{}",
        factorize(n)
            .into_iter()
            .map(|(_, v)| v)
            .sum::<usize>()
            .next_power_of_two()
            .trailing_zeros()
    );
}

fn factorize(mut n: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for i in 2..=n.sqrt() {
        if n % i != 0 {
            continue;
        }
        let mut ex = 0;
        while n % i == 0 {
            ex += 1;
            n /= i;
        }
        res.push((i, ex));
    }
    if n != 1 {
        res.push((n, 1));
    }
    res
}
