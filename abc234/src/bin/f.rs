use proconio::{input, marker::Chars};

const MOD: usize = 998_244_353;

fn main() {
    input! {
       mut s:Chars,
    }
    let n = s.len();
    s.sort();
    let mut map = vec![0; 26];
    for e in s {
        map[(e as u8 - b'a') as usize] += 1;
    }
    let binom = Binom::init(n, MOD);

    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 0..26 {
        let mut next = vec![0; n + 1];
        for j in 0..=n {
            for k in 0..=j.min(map[i]) {
                next[j] += dp[j - k] * binom.get(j, k);
                next[j] %= MOD;
            }
        }
        dp = next;
    }
    dp.remove(0);
    println!("{}", dp.iter().fold(0, |acc, x| (acc + x) % MOD));
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
