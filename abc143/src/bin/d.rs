use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        mut l:[i64;n],
    }
    l.sort();
    let mut ans = 0;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            let min = (l.upper_bound(&(l[j] - l[i])) + 1).max(j + 1);
            let max = l.lower_bound(&(l[i] + l[j]));
            ans += max.saturating_sub(min);
        }
    }
    println!("{}", ans);
}
