use proconio::{input, marker::Usize1};

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
        uv:[(Usize1,Usize1);m],
    }
    let mut odd = vec![false; n];
    for (u, v) in uv {
        odd[u] = !odd[u];
        odd[v] = !odd[v];
    }
    let o_cnt = odd.iter().filter(|b| **b).count();
    let e_cnt = n - o_cnt;
    let mut ans = 0;
    let comb = Binom::init(n, MOD);
    for i in (0..=k).step_by(2) {
        if o_cnt < i || e_cnt < k - i {
            continue;
        }
        ans += comb.get(o_cnt, i) * comb.get(e_cnt, k - i);
        ans %= MOD;
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
