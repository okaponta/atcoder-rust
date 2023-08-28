use proconio::{input, marker::Chars};

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
       k:usize,
       s:Chars,
    }
    let n = s.len();
    let c = Binom::init(n + k, MOD);
    let mut tsix = vec![1];
    let mut tfive = vec![1];
    for i in 0..k {
        tsix.push(tsix[i] * 26 % MOD);
        tfive.push(tfive[i] * 25 % MOD);
    }
    let mut ans = 0;
    for i in 0..=k {
        ans = ans + tsix[k - i] * tfive[i] % MOD * c.get(n - 1 + i, i) % MOD;
        ans %= MOD;
    }
    println!("{}", ans);
}

pub struct Binom {
    fact: Vec<usize>,
    fact_inv: Vec<usize>,
    modulo: usize,
}

impl Binom {
    pub fn init(n: usize, modulo: usize) -> Self {
        let mut fact = vec![0; n + 1];
        let mut inv = vec![0; n + 1];
        let mut fact_inv = vec![0; n + 1];
        fact[0] = 1;
        fact[1] = 1;
        inv[1] = 1;
        fact_inv[0] = 1;
        fact_inv[1] = 1;
        for i in 2..(n + 1) {
            fact[i] = fact[i - 1] * i % modulo;
            inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo;
            fact_inv[i] = fact_inv[i - 1] * inv[i] % modulo;
        }
        Self {
            fact,
            fact_inv,
            modulo,
        }
    }

    // nCk
    pub fn get(&self, n: usize, k: usize) -> usize {
        assert!(n >= k);
        self.fact[n] * self.fact_inv[k] % self.modulo * self.fact_inv[n - k] % self.modulo
    }
}
