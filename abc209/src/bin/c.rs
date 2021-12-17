use proconio::input;

const MOD: i64 = 1000000007;

fn main() {
    input! {
       n:usize, mut c:[i64;n],
    }
    c.sort();
    let mut ans: i64 = 1;
    for i in 0..n {
        ans = (ans * (c[i] - i as i64)) % MOD;
    }
    println!("{}", ans);
}
