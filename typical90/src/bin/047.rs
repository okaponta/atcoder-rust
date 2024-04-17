use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
        t:Chars,
    }
    let mut t1 = vec![];
    let mut t2 = vec![];
    let mut t3 = vec![];
    for i in 0..n {
        if t[i] == 'R' {
            t1.push('R');
            t2.push('B');
            t3.push('G');
        } else if t[i] == 'G' {
            t1.push('B');
            t2.push('G');
            t3.push('R');
        } else {
            t1.push('G');
            t2.push('R');
            t3.push('B');
        }
    }
    const MOD: u128 = 1_000_000_007;
    const BASE: u128 = 10007;
    let hs = ModRollingHash::init(MOD, BASE, n, &s);
    let ht = vec![
        ModRollingHash::init(MOD, BASE, n, &t1),
        ModRollingHash::init(MOD, BASE, n, &t2),
        ModRollingHash::init(MOD, BASE, n, &t3),
    ];
    let mut ans = 0;
    for i in 1..=n {
        if ht
            .iter()
            .any(|ht| (hs.hash(0, i) * hs.b[n - i]) % MOD == ht.hash(n - i, n))
        {
            ans += 1;
        }
        if ht
            .iter()
            .any(|ht| hs.hash(n - i, n) == (ht.hash(0, i) * (hs.b[n - i])) % MOD)
        {
            ans += 1;
        }
    }
    if ht.iter().any(|ht| hs.hash(0, n) == ht.hash(0, n)) {
        ans -= 1;
    }
    println!("{}", ans);
}

pub struct ModRollingHash {
    modulo: u128,
    b: Vec<u128>,
    hash: Vec<u128>,
}

impl ModRollingHash {
    // modulo: u128
    // const BASE:u128 = 1000001137;
    pub fn init(modulo: u128, base: u128, n: usize, str: &Vec<char>) -> Self {
        let mut b = vec![1u128];
        let mut hash = vec![0u128];
        let mut t = 1;
        for i in 0..n {
            hash.push((((str[i] as u128 * t) % modulo) + hash[i]) % modulo);
            t = (t * base) % modulo;
            b.push(t);
        }
        Self { modulo, b, hash }
    }

    // i..jまでの文字列のハッシュ値
    // 位置が違うところで比較するときは、b[差分]だけかけてから比較すること
    fn hash(&self, i: usize, j: usize) -> u128 {
        (self.modulo + self.hash[j] - self.hash[i]) % self.modulo
    }
}
