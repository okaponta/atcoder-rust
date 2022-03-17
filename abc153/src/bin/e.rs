use proconio::input;

fn main() {
    input! {
        h:usize, n:usize,
        ab:[(usize,usize);n],
    }
    let mut ans = vec![1 << 60; h + 10002];
    ans[0] = 0;
    for i in 0..h {
        for &(a, b) in &ab {
            ans[i + a] = ans[i + a].min(ans[i] + b);
        }
    }
    let mut a = 1 << 60;
    for i in h..h + 10001 {
        a = a.min(ans[i]);
    }
    println!("{}", a);
}
