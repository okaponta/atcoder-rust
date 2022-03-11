use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s: Chars
    }
    let mut r_cnt = 0i64;
    let mut g_cnt = 0i64;
    let mut b_cnt = 0i64;
    let mut rgb = vec![];
    for i in 0..n {
        match s[i] {
            'R' => {
                r_cnt += 1;
                rgb.push(1);
            }
            'G' => {
                g_cnt += 1;
                rgb.push(2);
            }
            _ => {
                b_cnt += 1;
                rgb.push(4);
            }
        }
    }
    let mut ans = r_cnt * g_cnt * b_cnt;
    for i in 0..n {
        for j in i + 1..n {
            let h = 2 * j - i;
            if h >= n {
                continue;
            }
            if rgb[i] + rgb[j] + rgb[h] == 7 {
                ans -= 1;
            }
        }
    }
    println!("{}", ans);
}
