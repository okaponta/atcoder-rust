use proconio::input;

fn main() {
    input! {
        n:usize,
        mut h:[usize;n]
    }
    h.push(1 << 60);
    let mut ans = 0;
    let mut temp = 0;
    for i in 1..=n {
        if h[i - 1] >= h[i] {
            temp += 1;
        } else {
            ans = ans.max(temp);
            temp = 0;
        }
    }
    println!("{}", ans);
}
