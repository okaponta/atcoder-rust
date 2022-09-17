use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        _:usize,
        m:usize,
        q:usize,
        abcd:[(usize,usize,usize,usize);q],
    }
    for (a, b, c, d) in abcd {
        let mut ans = 0;
        let (a1, b1, c1, d1) = trim((a, b, c, d), 1);
        if a1 <= b1 && c1 <= d1 {
            ans += sum(a1, b1, c1, d1, m);
        }
        let (a2, b2, c2, d2) = trim((a, b, c, d), 0);
        if a2 <= b2 && c2 <= d2 {
            ans += sum(a2, b2, c2, d2, m);
        }
        println!("{}", ans % MOD);
    }
}

fn tousa_sum_term(a: usize, c: usize, n: usize) -> usize {
    (a * n % MOD + (n * (n - 1) / 2) % MOD * c) % MOD
}

fn trim(abcd: (usize, usize, usize, usize), n: usize) -> (usize, usize, usize, usize) {
    let (mut a, mut b, mut c, mut d) = abcd;
    if a % 2 != n {
        a += 1;
    }
    if c % 2 != n {
        c += 1;
    }
    if b % 2 != n {
        b -= 1;
    }
    if d % 2 != n {
        d -= 1;
    }
    (a, b, c, d)
}

fn sum(a: usize, b: usize, c: usize, d: usize, m: usize) -> usize {
    let first = (c + (a - 1) * m) % MOD;
    let cols = ((d - c) / 2 + 1) % MOD;
    let first_row = tousa_sum_term(first, 2, cols) % MOD;
    let rows = ((b - a) / 2 + 1) % MOD;
    tousa_sum_term(first_row, cols * 2 * m % MOD, rows)
}
