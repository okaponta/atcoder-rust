use proconio::input;

fn main() {
    input! {
        mut n:usize,
        mut c:[usize;9],
    }
    let mn = *c.iter().min().unwrap();
    c.insert(0, mn);
    let len = n / mn;
    n -= mn * len;
    for i in 0..10 {
        c[i] -= mn;
    }
    let mut ans = String::new();
    for _ in 0..len {
        for j in (1..10).rev() {
            if c[j] <= n {
                n -= c[j];
                ans.push((b'0' + j as u8) as char);
                break;
            }
        }
    }
    println!("{}", ans);
}
