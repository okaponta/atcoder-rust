use proconio::input;

fn main() {
    input! {
        n:usize,
        s:usize,
        k:usize,
        pq:[(usize,usize);n],
    }
    let mut ans = (0..n).into_iter().map(|i| pq[i].0 * pq[i].1).sum::<usize>();
    if ans < s {
        ans += k;
    }
    println!("{}", ans);
}
