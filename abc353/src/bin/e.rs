use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        mut s:[Chars;n],
    }
    s.sort();
    println!("{}", dfs(&s, 0, n - 1, 0));
}

fn dfs(s: &Vec<Vec<char>>, from: usize, to: usize, letters: usize) -> usize {
    let mut res = 0;
    let mut used = vec![false; 26];
    let mut fr = vec![0; 26];
    let mut t = vec![0; 26];
    let mut cnt = vec![0; 26];
    for i in from..=to {
        if s[i].len() == letters {
            continue;
        }
        let c = (s[i][letters] as u8 - b'a') as usize;
        if !used[c] {
            fr[c] = i;
            used[c] = true;
        }
        t[c] = i;
        cnt[c] += 1;
    }
    for i in 0..26 {
        if cnt[i] != 0 {
            res += cnt[i] * (cnt[i] - 1) / 2;
            res += dfs(s, fr[i], t[i], letters + 1);
        }
    }
    res
}
