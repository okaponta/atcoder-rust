use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut ans = 0;
    for l in 0..n {
        let mut x = a[l];
        for r in l..n {
            x = x.min(a[r]);
            ans = ans.max(x * (r - l + 1));
        }
    }
    println!("{}", ans);
}
