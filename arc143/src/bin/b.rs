use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,
    }
    let mut fact = vec![1];
    for i in 1..=n * n {
        fact.push((fact[i - 1] * i) % MOD);
    }
    let comb = Binom::init(n * n, MOD);
    let mut ans = fact[n * n];
    let small_sq = (n - 1) * (n - 1);
    for k in 0..=small_sq {
        let tmp = n * comb.get(n + k - 1, n - 1) % MOD * fact[n] % MOD * comb.get(small_sq, k)
            % MOD
            * fact[k]
            % MOD
            * fact[n * (n - 1) - k]
            % MOD;
        ans = (ans + MOD - tmp) % MOD;
    }
    println!("{}", ans);
}

struct Binom {
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
