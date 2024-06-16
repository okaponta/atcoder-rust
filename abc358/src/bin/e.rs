use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        k:usize,
        c:[usize;26],
    }
    let comb = Binom::init(k + 1, MOD);
    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for i in 0..26 {
        let mut next = vec![0; k + 1];
        for j in 0..=k {
            for l in 0..=c[i] {
                if k < j + l {
                    break;
                }
                next[j + l] += dp[j] * comb.get(j + l, l);
                next[j + l] %= MOD;
            }
        }
        dp = next;
    }
    println!("{}", (1..=k).fold(0, |s, i| (s + dp[i]) % MOD));
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
