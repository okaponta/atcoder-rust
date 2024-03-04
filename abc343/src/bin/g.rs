use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut n:usize,
        mut s:[Chars;n],
    }
    // 完全に含むやつがあったら消す
    for i in (0..n).rev() {
        if (0..n).filter(|j| j != &i).any(|j| contains(&s[j], &s[i])) {
            s.remove(i);
            n -= 1;
        }
    }
    // siの末尾とsjの先頭が連続する数 -> cij
    let mut common = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let sji = s[j].iter().chain(s[i].iter()).collect::<Vec<_>>();
            let z = z(&sji);
            let mut max = 0;
            for k in s[j].len()..s[i].len() + s[j].len() {
                if s[i].len() - (k - s[j].len()) == z[k] {
                    max = max.max(z[k]);
                }
            }
            common[i][j] = max.min(s[i].len());
        }
    }
    let mut dp = vec![vec![1 << 60; n]; 1 << n];
    for i in 0..n {
        dp[1 << i][i] = s[i].len();
    }
    for i in 1..1 << n {
        for j in 0..n {
            if i >> j & 1 == 0 {
                // 未訪問
                continue;
            }
            for k in 0..n {
                if i >> k & 1 == 1 {
                    // 訪問済
                    continue;
                }
                dp[i | 1 << k][k] = dp[i | 1 << k][k].min(dp[i][j] + s[k].len() - common[j][k]);
            }
        }
    }
    let mut ans = 1 << 60;
    for i in 0..n {
        ans = ans.min(dp[(1 << n) - 1][i]);
    }
    println!("{}", ans);
}

// sの中にtはあるか？
fn contains(s: &Vec<char>, t: &Vec<char>) -> bool {
    let ts = t.iter().chain(s.iter()).collect::<Vec<_>>();
    let z = z(&ts);
    for i in t.len()..ts.len() {
        if t.len() <= z[i] {
            return true;
        }
    }
    false
}

// 最長共通接頭辞の長さ
// 例えば、hohohehoiなら
// 9,0,2,0,1,0,2,0,0
fn z(s: &Vec<&char>) -> Vec<usize> {
    let n = s.len();
    let mut res = vec![0; n + 1];
    res[0] = n;
    let mut i = 1;
    let mut j = 0;
    while i < n {
        while i + j < n && s[j] == s[i + j] {
            j += 1;
        }
        res[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while k < j && k + res[k] < j {
            res[i + k] = res[k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    res
}
