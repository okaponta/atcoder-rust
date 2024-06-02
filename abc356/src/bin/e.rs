use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    a.sort();
    let mut c = vec![0; 100];
    let mut ans = 0;
    let mut imos = vec![0; 1_000_001];
    let mut tmp = 1;
    for i in 0..n {
        for j in 1..100 {
            ans += (a[i] / j) * c[j];
        }
        if a[i] < 100 {
            c[a[i]] += 1;
        } else {
            if tmp < a[i] {
                for j in tmp + 1..=a[i] {
                    imos[j] += imos[j - 1];
                }
                tmp = a[i];
            }
            ans += imos[a[i]];
            for j in (a[i]..1_000_001).step_by(a[i]) {
                imos[j] += 1;
            }
        }
    }
    println!("{}", ans);
}
