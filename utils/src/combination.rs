// 二項定理 nまでのiCjを計算して返却する。
// MODが素数である必要あり。
// 計算量はO(N)
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

// 二項定理 nまでのiCjを計算して返却する。(k<10^7,n<10^9と巨大のとき)
// MODが素数である必要あり。
// 計算量はO(K)/クエリもO(K)
struct Com {
    fact_inv: Vec<usize>,
    modulo: usize,
}

impl Com {
    pub fn init(k: usize, modulo: usize) -> Self {
        let mut inv = vec![0; k + 1];
        let mut fact_inv = vec![0; k + 1];
        inv[1] = 1;
        fact_inv[0] = 1;
        fact_inv[1] = 1;
        for i in 2..(k + 1) {
            inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo;
            fact_inv[i] = fact_inv[i - 1] * inv[i] % modulo;
        }
        Self { fact_inv, modulo }
    }

    // nCk
    pub fn get(&self, n: usize, k: usize) -> usize {
        assert!(n >= k);
        let mut res = 1;
        for i in 0..k {
            res *= n - i;
            res %= self.modulo;
        }
        res * self.fact_inv[k] % self.modulo
    }
}

// 二項定理 nまでのiCjを計算して返却する。(k<10^7,n<10^9と巨大のとき)
// MODが素数である必要あり。Nが固定値の場合に使える。
// 計算量はO(K)/クエリはO(1)
struct Comb {
    c: Vec<usize>,
    modulo: usize,
}

impl Comb {
    pub fn init(n: usize, k: usize, modulo: usize) -> Self {
        let mut c = vec![0; k + 1];
        c[0] = 1;
        c[1] = n;
        for i in 2..(k + 1) {
            c[i] = (c[i - 1] * (n - i + 1) / i) % modulo;
        }
        Self { c, modulo }
    }

    // nCk
    pub fn get(&self, k: usize) -> usize {
        self.c[k]
    }
}

// 二項定理 nまでのiCjを計算して返却する。
// MODが素数じゃなくても有効。
// 計算量はO(N^2)
fn binom_init(n: usize, modulo: usize) -> Vec<Vec<usize>> {
    let mut res = vec![vec![0; n + 1]; n + 1];
    res[0][0] = 1;
    for i in 1..=n {
        res[i][0] = 1;
        res[i][i] = 1;
        for j in 1..i {
            res[i][j] = (res[i - 1][j - 1] + res[i - 1][j]) % modulo;
        }
    }
    res
}

// 単体でnCkを求めたいとき
fn binom(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    (0..k).fold(1, |s, k| s * (n - k) / (k + 1))
}

// nCkの偶奇だけ知りたいとき
fn is_nck_odd(n: usize, k: usize) -> bool {
    n & k == k
}
