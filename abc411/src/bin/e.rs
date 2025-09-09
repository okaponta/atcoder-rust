#[rustfmt::skip]
use {proconio::*,std::collections::*,itertools::*,ac_library::ModInt998244353 as M};

fn main() {
    input! {
        n:usize,
        a:[[usize;6];n],
    }
    let mut map = HashMap::new();
    let mut v = vec![];
    for (i, j) in iproduct!(0..n, 0..6) {
        map.entry(a[i][j]).or_insert(vec![]).push(i);
        v.push(a[i][j]);
    }
    map.insert(0, vec![]);
    v.sort();
    v.dedup();
    let m = v.len();
    v.insert(0, 0);
    let mut ans = M::new(0);
    let mut cnt = vec![0; n];
    let inv = (1..7).into_iter().map(|i| M::new(i).inv()).collect_vec();
    let mut o1 = HashSet::new();
    let mut pp = inv[5].pow(n as u64);
    for i in 1..=m {
        for &j in &map[&v[i - 1]] {
            if cnt[j] != 0 {
                pp *= inv[cnt[j] - 1] * M::new(cnt[j] + 1);
            } else {
                o1.insert(j);
            }
            cnt[j] += 1;
        }
        if o1.len() != n {
            ans += M::new(1) * M::new(v[i] - v[i - 1]);
        } else {
            ans += (M::new(1) - pp) * M::new(v[i] - v[i - 1]);
        }
    }
    println!("{}", ans)
}
