use proconio::input;

fn main() {
    input! {
        n:usize,mut w:i64,
        mut ab:[(i64,i64);n]
    }
    ab.sort_by(|&a, &b| b.0.cmp(&a.0));
    let mut ans = 0;
    for i in 0..n {
        if ab[i].1 <= w {
            w -= ab[i].1;
            ans += ab[i].0 * ab[i].1;
        } else {
            ans += w * ab[i].0;
            break;
        }
    }
    println!("{}", ans);
}
