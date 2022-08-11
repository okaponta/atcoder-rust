use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
        d:usize,
        x:[[i64;d];n]
    }
    let mut count = 0;
    for i in 0..n {
        for j in i + 1..n {
            if is_sq(dist(&x[i], &x[j])) {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn dist(a: &Vec<i64>, b: &Vec<i64>) -> i64 {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| (a - b).pow(2))
        .sum::<i64>()
}

fn is_sq(i: i64) -> bool {
    i.sqrt() * i.sqrt() == i
}
