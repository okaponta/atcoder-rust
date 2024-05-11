use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut c = vec![];
    let mut count = vec![0; 11];
    for i in 0..n {
        let mut tmp = a[i];
        let mut d = 1;
        while 9 < tmp {
            tmp /= 10;
            d += 1;
        }
        count[d] += 1;
        c.push(d);
    }

    let s = ruiseki(&a);
    let mut ans = 0;
    for i in 0..n {
        count[c[i]] -= 1;
        ans += calc(&count) * a[i];
        ans %= 998244353;
        ans += s[n] - s[i + 1];
        ans %= 998244353;
    }
    println!("{}", ans);
}

fn calc(c: &Vec<usize>) -> usize {
    let mut res = 0;
    let mut tmp = 1;
    for i in 0..11 {
        res += c[i] * tmp;
        res %= 998244353;
        tmp *= 10;
        tmp %= 998244353;
    }
    res
}

// 累積和
fn ruiseki(a: &Vec<usize>) -> Vec<usize> {
    let mut res = vec![0];
    for i in 0..a.len() {
        res.push(res[i] + a[i]);
    }
    res
}
