use proconio::input;

fn main() {
    input! {
       l:usize,
    }
    println!("{}", binom(l - 1, 11));
}

fn binom(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    (0..k).fold(1, |s, k| s * (n - k) / (k + 1))
}
