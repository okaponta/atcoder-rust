use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        c:usize,
        stc:[(usize,usize,Usize1);n],
    }
    let n = (0..n).into_iter().map(|i| stc[i].1).max().unwrap();
    let mut rec = vec![vec![false; n + 1]; c];
    for (s, t, c) in stc {
        for i in s..=t {
            rec[c][i] = true;
        }
    }
    let mut ans = 0;
    for i in 0..=n {
        let mut count = 0;
        for j in 0..c {
            if rec[j][i] {
                count += 1;
            }
        }
        ans = ans.max(count);
    }
    println!("{}", ans);
}
