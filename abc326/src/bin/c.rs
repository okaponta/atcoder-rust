use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut a:[usize;n],
    }
    a.sort();
    let mut ans = 0;
    for i in 0..n {
        let tmp = a.lower_bound(&(a[i] + m));
        ans = ans.max(tmp - i);
    }
    println!("{}", ans);
}
