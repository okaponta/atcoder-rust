use proconio::input;

fn main() {
    input! {
        n:usize,
        a:usize,
        mut b:usize,
        mut v:[usize;n],
    }
    b -= a;
    v.sort();
    let mut max = vec![];
    for _i in 0..a {
        max.push(v.pop().unwrap());
    }
    let i1 = max.iter().min().unwrap();
    println!(
        "{}",
        max.iter().map(|i| *i as f64).sum::<f64>() / max.len() as f64
    );
    let mut n1 = max.iter().filter(|&i| i == i1).count();
    let mut n2 = v.iter().filter(|&i| i == i1).count();
    let mut ans = binom(n1 + n2, n2);
    if !max.iter().all(|i| i == i1) {
        println!("{}", ans);
        return;
    }
    for _ in 0..b {
        if n2 == 0 {
            break;
        }
        n1 += 1;
        n2 -= 1;
        ans += binom(n1 + n2, n2);
    }
    println!("{}", ans);
}

fn binom(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    (0..k).fold(1, |s, k| s * (n - k) / (k + 1))
}
