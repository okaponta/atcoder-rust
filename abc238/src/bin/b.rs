use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i32;n],
    }
    let mut cut = vec![false; 360];
    cut[0] = true;
    let mut deg = 0;
    for i in 0..n {
        deg = (deg + a[i]) % 360;
        cut[deg as usize] = true;
    }
    let mut ans = 0;
    let mut tmp = 0;
    for c in cut {
        tmp += 1;
        if c {
            ans = ans.max(tmp);
            tmp = 0;
        }
    }
    tmp += 1;
    ans = ans.max(tmp);
    println!("{}", ans);
}
