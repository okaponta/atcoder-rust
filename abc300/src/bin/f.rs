use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
        s:Chars,
    }
    let mut x_c = 0;
    let mut cum = vec![0];
    for i in 0..n {
        if s[i] == 'x' {
            x_c += 1;
        }
        cum.push(x_c);
    }
    println!(
        "{}",
        (0..n)
            .into_iter()
            .map(|i| max_right(i, m, x_c, n, k, &cum) - i)
            .max()
            .unwrap_or(0)
    );
}

fn max_right(start: usize, m: usize, count: usize, n: usize, k: usize, s: &Vec<usize>) -> usize {
    let mut lower = start;
    let mut upper = n * m;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if med / n * count + s[med % n + 1] - s[start] <= k {
            lower = med;
        } else {
            upper = med;
        }
    }
    upper
}
