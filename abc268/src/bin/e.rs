use proconio::input;

fn main() {
    input! {
        n:usize,
        p:[usize;n],
    }
    // 何回動かせば不満が0になるか
    let mut t = vec![0; n];
    for i in 0..n {
        t[i] = (n + p[i] - i) % n;
    }
    // imos法で一次の係数と定数を保持する
    let mut f_imos = vec![0; 2 * n];
    let mut z_imos = vec![0; 2 * n];
    for begin in t {
        let change = begin + (n + 1) / 2;
        let end = begin + n;
        f_imos[begin] += 1;
        z_imos[begin] -= begin as i64;
        f_imos[change] -= 2;
        z_imos[change] += 2 * begin as i64 + n as i64;
        f_imos[end] += 1;
        z_imos[end] -= begin as i64 + n as i64;
    }
    let mut ans = vec![0; n];
    let mut first = 0;
    let mut zero = 0;
    for i in 0..2 * n {
        first += f_imos[i];
        zero += z_imos[i];
        ans[i % n] += first * i as i64 + zero;
    }
    println!("{}", ans.iter().min().unwrap())
}
