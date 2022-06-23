use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut grid = vec![0; 4];
    let mut idx = 0;
    let mut ans = n.saturating_sub(4);
    for i in n.saturating_sub(4)..n {
        grid[idx] = a[i];
        idx += 1;
    }
    let mut s = 0;
    for i in (0..4).rev() {
        s += grid[i];
        if 3 < s {
            ans += 1;
        }
    }
    println!("{}", ans);
}
