use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        a:[[u8;4];4],
    }
    let mut count = 0;
    for s in 0..(1 << 16) {
        let mut b = vec![vec![0; 4]; 4];
        for i in 0..16 {
            if (s >> i) & 1 == 1 {
                b[i / 4][i % 4] = 1;
            }
        }
        // 街を囲っているかチェック
        let is_surrounding_vil = (0..16usize)
            .into_iter()
            .filter(|i| a[i / 4][i % 4] == 1 && b[i / 4][i % 4] == 0)
            .count()
            == 0;
        if !is_surrounding_vil {
            continue;
        }
        // 連結チェック
        if !is_renketsu(&mut b, 1, 4) {
            continue;
        }
        // 穴がないかチェック
        if !has_no_hole(&mut b, 0, 4) {
            continue;
        }
        count += 1;
    }
    println!("{}", count);
}

// 連結であるかをチェックする
// c..チェック対象の数字
// n..盤面の大きさ
fn is_renketsu(b: &Vec<Vec<usize>>, c: usize, n: usize) -> bool {
    let mut q = VecDeque::new();
    for i in 0..(n * n) {
        if b[i / n][i % n] == 1 {
            q.push_back((i / n, i % n));
            break;
        }
    }
    let mut visited = vec![vec![false; n]; n];
    while let Some((x, y)) = q.pop_front() {
        visited[x][y] = true;

        for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
            let xi = x.wrapping_add(dx);
            let yi = y.wrapping_add(dy);
            if n <= xi || n <= yi {
                continue;
            }
            if b[xi][yi] == c && !visited[xi][yi] {
                q.push_back((xi, yi));
            }
        }
    }

    (0..n).all(|i| (0..n).all(|j| b[i][j] != c || visited[i][j]))
}

// 穴があるかをチェックする(外から連結であるかをチェックします)
// c..チェック対象の数字(cじゃないものを判定します)
// n..盤面の大きさ
fn has_no_hole(b: &Vec<Vec<usize>>, c: usize, n: usize) -> bool {
    let mut q = VecDeque::new();
    for i in 0..n {
        if b[i][0] == 0 {
            q.push_back((i, 0));
        }
        if b[i][n - 1] == 0 {
            q.push_back((i, n - 1));
        }
        if b[0][i] == 0 {
            q.push_back((0, i));
        }
        if b[n - 1][i] == 0 {
            q.push_back((n - 1, i));
        }
    }
    let mut visited = vec![vec![false; n]; n];
    while let Some((x, y)) = q.pop_front() {
        visited[x][y] = true;

        for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
            let xi = x.wrapping_add(dx);
            let yi = y.wrapping_add(dy);
            if n <= xi || n <= yi {
                continue;
            }
            if b[xi][yi] == c && !visited[xi][yi] {
                q.push_back((xi, yi));
            }
        }
    }

    (0..n).all(|i| (0..n).all(|j| b[i][j] != c || visited[i][j]))
}
