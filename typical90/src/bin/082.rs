use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        l:usize,
        r:usize,
    }
    println!("{}", (f(r) + MOD - f(l - 1)) % MOD);
}

fn f(a: usize) -> usize {
    if a == 0 {
        return 0;
    }
    let mut tmp = 1;
    let mut num = 1;
    while tmp * 10 <= a {
        tmp *= 10;
        num += 1;
    }
    let c = (kai(a) + MOD - kai(tmp - 1)) % MOD;
    ((num % MOD) * c + f(tmp - 1)) % MOD
}

fn kai(a: usize) -> usize {
    ((a % MOD) * ((a + 1) % MOD) / 2) % MOD
}
