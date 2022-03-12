use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,w:usize,
        s:[Chars;h],
    }
    let mut is_road = vec![vec![false; w]; h];
    for (i, j) in iproduct!(0..h, 0..w) {
        if s[i][j] == '.' {
            is_road[i][j] = true;
        }
    }
    let mut ans = 0;
    // 始点
    for (i, j) in iproduct!(0..h, 0..w) {
        if !is_road[i][j] {
            continue;
        }
        let mut dist = vec![vec![1 << 60; w]; h];
        dfs(
            &mut dist, &is_road, i as i32, j as i32, 0, h as i32, w as i32,
        );
        let mut tmp_max = 0;
        for (i, j) in iproduct!(0..h, 0..w) {
            if dist[i][j] == 1 << 60 {
                continue;
            }
            tmp_max = tmp_max.max(dist[i][j]);
        }
        ans = ans.max(tmp_max);
    }
    println!("{}", ans);
}

fn dfs(
    dist: &mut Vec<Vec<usize>>,
    is_road: &Vec<Vec<bool>>,
    cur_x: i32,
    cur_y: i32,
    d: usize,
    h: i32,
    w: i32,
) {
    if cur_x < 0 || h <= cur_x || cur_y < 0 || w <= cur_y {
        return;
    }
    if !is_road[cur_x as usize][cur_y as usize] {
        return;
    }
    if dist[cur_x as usize][cur_y as usize] <= d {
        return;
    }
    dist[cur_x as usize][cur_y as usize] = d;
    dfs(dist, is_road, cur_x + 1, cur_y, d + 1, h, w);
    dfs(dist, is_road, cur_x, cur_y + 1, d + 1, h, w);
    dfs(dist, is_road, cur_x - 1, cur_y, d + 1, h, w);
    dfs(dist, is_road, cur_x, cur_y - 1, d + 1, h, w);
}
