use num_integer::Roots;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        q:usize,
        lr:[(Usize1,usize);q],
    }
    let p = eratosthenes(100000);
    let mut s = vec![0; 100000];
    for i in (3..100000).step_by(2) {
        s[i - 1] = s[i - 2];
        s[i] = s[i - 2];
        if p[i] == 0 && p[(i + 1) / 2] == 0 {
            s[i] += 1;
        }
    }
    for (l, r) in lr {
        println!("{}", s[r] - s[l]);
    }
}

fn eratosthenes(n: usize) -> Vec<usize> {
    let mut res = vec![0; n + 1];
    res[0] = 0;
    res[1] = 1;
    for i in 2..=n.sqrt() {
        if res[i] != 0 {
            continue;
        }
        for j in (i * i..=n).step_by(i) {
            res[j] = i;
        }
    }
    res
}
