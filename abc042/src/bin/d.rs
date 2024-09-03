use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        a:usize,
        b:usize,
    }
    const MOD: usize = 1_000_000_007;
    let comb = Binom::init(h + w, MOD);
    let mut ans = 0;
    for i in b..w {
        ans += comb.get(h - a - 1 + i, i) * comb.get(w - i + a - 2, w - 1 - i) % MOD;
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
