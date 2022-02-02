use proconio::input;

fn main() {
    input! {
        n:usize,m:usize,
        mut a:[usize;m],
    }
    a.push(0);
    a.push(n + 1);
    a.sort();
    a.dedup();
    let white = a
        .windows(2)
        .map(|e| e[1] - e[0] - 1)
        .filter(|e| e > &0)
        .collect::<Vec<_>>();
    if white.len() == 0 {
        println!("{}", 0);
        return;
    }
    let k = *white.iter().min().unwrap();
    let mut ans = 0;
    for wi in white {
        ans += (wi + k - 1) / k;
    }
    println!("{}", ans);
}
