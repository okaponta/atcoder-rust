use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        s:Chars,
        t:Chars,
    }
    let mut c = vec![vec![]; 26];
    for i in 0..s.len() {
        c[(s[i] as u8 - b'a') as usize].push(i + 1);
    }
    let mut lower = 0;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if is_ok(med, n, &c, &t) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", lower);
}

fn is_ok(med: usize, n: usize, cnt: &Vec<Vec<usize>>, t: &Vec<char>) -> bool {
    let mut a = 0;
    let mut idx = 0;
    for &c in t {
        let c = (c as u8 - b'a') as usize;
        if cnt[c].len() == 0 {
            return false;
        }
        let tmp = cnt[c].upper_bound(&idx);
        let mut rem = med;
        if rem <= cnt[c].len() - tmp {
            idx = cnt[c][tmp + rem - 1];
        } else {
            rem -= cnt[c].len() - tmp;
            a += 1;
            if rem % cnt[c].len() == 0 {
                a += (rem / cnt[c].len()) - 1;
                idx = cnt[c][cnt[c].len() - 1];
            } else {
                a += rem / cnt[c].len();
                rem %= cnt[c].len();
                idx = cnt[c][rem - 1];
            }
        }
        if n <= a {
            return false;
        }
    }
    true
}
