use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        k:usize,
        s:[Chars;h],
    }
    let mut ans = 1010;
    // 縦横逆にする
    let mut choco = vec![vec![false; h]; w];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '1' {
                choco[j][i] = true;
            }
        }
    }
    'bit: for i in 0usize..1 << (h - 1) {
        let mut div_cnt = 0;
        // ホワイトチョコの数
        let n = i.count_ones() as usize + 1;
        let mut count = vec![0; n];
        for j in 0..w {
            let mut cur_count = vec![0; n];
            let mut l = 0;
            for k in 0..h {
                if choco[j][k] {
                    cur_count[l] += 1;
                }
                if i >> k & 1 == 1 {
                    l += 1;
                }
            }
            if (0..n).into_iter().any(|i| k < cur_count[i]) {
                continue 'bit;
            }
            if (0..n).into_iter().all(|i| count[i] + cur_count[i] <= k) {
                for k in 0..n {
                    count[k] += cur_count[k];
                }
            } else {
                count = cur_count;
                div_cnt += 1;
            }
        }
        ans = ans.min(div_cnt + n - 1);
    }
    println!("{}", ans);
}
