use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
        b:[usize;n],
        c:[usize;n],
    }
    let mut ra = vec![0; 46];
    let mut rb = vec![0; 46];
    let mut rc = vec![0; 46];
    for a in a {
        ra[a % 46] += 1;
    }
    for b in b {
        rb[b % 46] += 1;
    }
    for c in c {
        rc[c % 46] += 1;
    }
    let mut ans = 0usize;
    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                if (i + j + k) % 46 != 0 {
                    continue;
                }
                ans += ra[i] * rb[j] * rc[k];
            }
        }
    }
    println!("{}", ans);
}
