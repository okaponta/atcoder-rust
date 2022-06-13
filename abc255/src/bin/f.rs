use std::process::exit;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        p:[Usize1;n],
        i:[Usize1;n],
    }
    let mut rev_i = vec![0; n];
    for j in 0..n {
        rev_i[i[j]] = j;
    }
    if p[0] != 0 {
        println!("-1");
        return;
    }
    let mut ans = vec![(0, 0); n];
    dfs((0, n - 1), (0, n - 1), &p, &rev_i, &mut ans);
    for (l, r) in ans {
        println!("{} {}", l, r);
    }
}

fn dfs(
    p_range: (usize, usize),
    i_range: (usize, usize),
    p: &Vec<usize>,
    rev_i: &Vec<usize>,
    ans: &mut Vec<(usize, usize)>,
) {
    let target = p[p_range.0];
    let target_index_i = rev_i[target];
    if target_index_i < i_range.0 || i_range.1 < target_index_i {
        println!("-1");
        exit(0);
    }
    let left_len = target_index_i - i_range.0;
    let right_len = i_range.1 - target_index_i;

    let ans_left;
    let ans_right;
    if left_len == 0 {
        ans_left = 0;
    } else {
        ans_left = p[p_range.0 + 1] + 1;
        let next_p_range = (p_range.0 + 1, p_range.0 + left_len);
        let next_i_range = (i_range.0, target_index_i - 1);
        dfs(next_p_range, next_i_range, p, rev_i, ans);
    }

    if right_len == 0 {
        ans_right = 0;
    } else {
        ans_right = p[p_range.0 + left_len + 1] + 1;
        let next_p_range = (p_range.0 + left_len + 1, p_range.1);
        let next_i_range = (target_index_i + 1, i_range.1);
        dfs(next_p_range, next_i_range, p, rev_i, ans);
    }

    ans[target] = (ans_left, ans_right);
}
