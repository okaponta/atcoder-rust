use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let mut x_count = vec![0; 26];
    let mut y_count = vec![0; 26];
    for i in 0..n {
        y_count[(s[i] as u8 - b'a') as usize] += 1;
    }
    let mut ans = 0;
    for i in 0..n {
        let c = (s[i] as u8 - b'a') as usize;
        x_count[c] += 1;
        y_count[c] -= 1;
        let mut tmp = 0;
        for j in 0..26 {
            if 0 < x_count[j] && 0 < y_count[j] {
                tmp += 1;
            }
        }
        ans = ans.max(tmp);
    }
    println!("{}", ans);
}
