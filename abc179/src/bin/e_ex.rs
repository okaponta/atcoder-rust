use proconio::*;

fn main() {
    input! {
        n: usize, x: usize, m: usize
    }

    let (mut t, mut i) = (0, x);
    let mut mod_table = vec![None; m + 1];
    let mut sum_table = vec![0; m + 1];
    let (t1, t2) = loop {
        if let Some(v) = mod_table[i] {
            break (v, t);
        }
        mod_table[i] = Some(t);
        sum_table[t + 1] = sum_table[t] + i;
        t += 1;
        i = i * i % m;
    };

    let ans = {
        if n < t1 {
            sum_table[t1]
        } else {
            sum_table[t1]
                + (sum_table[t2] - sum_table[t1]) * ((n - t1) / (t2 - t1))
                + (sum_table[t1 + (n - t1) % (t2 - t1)] - sum_table[t1])
        }
    };

    println!("{}", ans);
}
