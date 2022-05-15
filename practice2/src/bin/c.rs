use proconio::input;

fn main() {
    input! {
        t:usize,
        nmab:[(usize,usize,usize,usize);t],
    }
    for (n, m, a, b) in nmab {
        println!("{}", floor_sum(n, m, a, b));
    }
}

fn floor_sum(n: usize, m: usize, mut a: usize, mut b: usize) -> usize {
    let mut res = 0;
    if a >= m {
        res += n * (n - 1) * (a / m) / 2;
        a %= m;
    }
    if b >= m {
        res += n * (b / m);
        b %= m;
    }
    let max = a * n + b;
    if max >= m {
        res += floor_sum(max / m, a, m, max % m);
    }
    res
}
