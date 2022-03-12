use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,w:usize,
        s:[Chars;h],
    }
    let mut is_road = vec![vec![false; w + 2]; h + 2];
    for (i, j) in iproduct!(0..h, 0..w) {
        if s[i][j] == '.' {
            is_road[i + 1][j + 1] = true;
        }
    }
    let mut ans = 0;
    // 始点
    for (i, j) in iproduct!(1..=h, 1..=w) {
        if !is_road[i][j] {
            continue;
        }
        let mut dist = vec![vec![1 << 60; w + 2]; h + 2];
        dfs(&mut dist, &is_road, i, j, 0);
        let tmp_max = dist
            .iter()
            .map(|v| v.iter().filter(|&&i| i != 1 << 60).max().unwrap_or(&0))
            .max()
            .unwrap();
        ans = ans.max(*tmp_max);
    }
    println!("{}", ans);
}

fn dfs(dist: &mut Vec<Vec<usize>>, is_road: &Vec<Vec<bool>>, cur_x: usize, cur_y: usize, d: usize) {
    if !is_road[cur_x][cur_y] {
        return;
    }
    if dist[cur_x][cur_y] <= d {
        return;
    }
    dist[cur_x][cur_y] = d;
    dfs(dist, is_road, cur_x + 1, cur_y, d + 1);
    dfs(dist, is_road, cur_x, cur_y + 1, d + 1);
    dfs(dist, is_road, cur_x - 1, cur_y, d + 1);
    dfs(dist, is_road, cur_x, cur_y - 1, d + 1);
}
