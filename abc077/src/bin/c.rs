use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
        mut b:[usize;n],
        mut c:[usize;n],
    }
    a.sort();
    b.sort();
    c.sort();
    let mut bc = vec![0; n + 1];
    for i in (0..n).rev() {
        bc[i] = bc[i + 1] + n - c.upper_bound(&b[i]);
    }
    let ans = a.iter().fold(0, |s, a| s + bc[b.upper_bound(a)]);
    println!("{}", ans);
}
