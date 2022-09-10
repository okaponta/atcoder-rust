use proconio::input;

fn main() {
    input! {
        n:usize,
        p:[usize;n],
    }
    let mut ans = vec![0; n];
    for i in 0..n {
        let base = (n + p[i] - i) % n;
        let bef = (n + base - 1) % n;
        let after = (base + 1) % n;
        ans[base] += 1;
        ans[bef] += 1;
        ans[after] += 1;
    }
    println!("{}", ans.iter().max().unwrap_or(&0));
}
