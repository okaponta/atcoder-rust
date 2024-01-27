use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        x:[Usize1;m],
    }
    let mut len = 0;
    let mut imos = vec![0; n + 1];
    for i in 1..m {
        let prev = (x[i - 1]).min(x[i]);
        let next = (x[i - 1]).max(x[i]);
        let a = (next - prev) as i64;
        let b = n as i64 - a;
        if a < b {
            imos[prev] += b - a;
            imos[next] -= b - a;
        } else {
            imos[0] += a - b;
            imos[prev] -= a - b;
            imos[next] += a - b;
        }
        len += a.min(b);
    }
    let mut add = 1 << 60;
    let mut tmp = 0;
    for i in 0..n {
        tmp += imos[i];
        add = add.min(tmp);
    }
    println!("{}", len + add);
}
