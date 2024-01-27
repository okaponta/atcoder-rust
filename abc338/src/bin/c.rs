use proconio::input;

fn main() {
    input! {
        n:usize,
        q:[usize;n],
        a:[usize;n],
        b:[usize;n],
    }
    let mut ans = 0;
    for i in 0..=1_000_000 {
        if (0..n).into_iter().any(|j| q[j] < a[j] * i) {
            break;
        }
        let mut nb = 1_000_000;
        for j in 0..n {
            if 0 < b[j] {
                let tmp = (q[j] - a[j] * i) / b[j];
                nb = nb.min(tmp);
            }
        }
        ans = ans.max(i + nb);
    }
    println!("{}", ans);
}
