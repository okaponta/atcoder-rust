use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize,
        ab:[(usize,usize);n],
    }
    let mut ans = 1 << 60;
    let mut base = 0;
    for i in 0..n {
        let tmp = base + ab[i].0 + (ab[i].1) * (x - i);
        ans = ans.min(tmp);
        base += ab[i].0 + ab[i].1;
    }
    println!("{}", ans);
}
