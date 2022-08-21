use proconio::input;

fn main() {
    input! {
        n:usize,
        p:usize,
        q:usize,
        r:usize,
        a:[usize;n],
    }
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }
    for i in 0..n {
        let base = s[i];
        if let Ok(_) = s.binary_search(&(base + p)) {
            if let Ok(_) = s.binary_search(&(base + p + q)) {
                if let Ok(_) = s.binary_search(&(base + p + q + r)) {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
