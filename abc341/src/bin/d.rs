use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
    }
    let l = lcm(n, m);
    let mut lower = 0;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if rank(med, n, m, l) < k {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", upper);
}

fn rank(med: usize, n: usize, m: usize, l: usize) -> usize {
    let nn = med / n;
    let nm = med / m;
    let nl = med / l;
    nn + nm - 2 * nl
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
