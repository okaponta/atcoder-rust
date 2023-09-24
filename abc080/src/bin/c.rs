use proconio::input;

fn main() {
    input! {
        n:usize,
        f:[[u8;10];n],
        p:[[i64;11];n],
    }
    let mut ans = -1 << 60;
    for i in 1..1 << 10 {
        let mut inter = vec![0; n];
        for j in 0..10 {
            if i >> j & 1 == 1 {
                for k in 0..n {
                    if f[k][j] == 1 {
                        inter[k] += 1;
                    }
                }
            }
        }
        let mut tmp = 0;
        for j in 0..n {
            tmp += p[j][inter[j]];
        }
        ans = ans.max(tmp);
    }
    println!("{}", ans);
}
