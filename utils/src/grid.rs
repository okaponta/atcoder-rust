use std::collections::VecDeque;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_renketsu_ok() {
        let n = 4;
        let b = vec![
            vec![0, 1, 1, 1],
            vec![0, 0, 1, 1],
            vec![1, 0, 0, 1],
            vec![1, 1, 1, 1],
        ];
        assert_eq!(is_renketsu(&b, 0, n), true);
        assert_eq!(is_renketsu(&b, 1, n), true);
    }

    #[test]
    fn test_is_renketsu_ng() {
        let n = 5;
        let b = vec![
            vec![0, 1, 1, 1, 0],
            vec![0, 0, 1, 1, 0],
            vec![1, 0, 0, 1, 0],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
        ];
        assert_eq!(is_renketsu(&b, 0, n), false);
        assert_eq!(is_renketsu(&b, 1, n), true);
    }

    #[test]
    fn test_has_no_hole_ok() {
        let n = 4;
        let b = vec![
            vec![0, 1, 1, 1],
            vec![0, 0, 1, 1],
            vec![1, 0, 0, 1],
            vec![1, 1, 1, 1],
        ];
        assert_eq!(has_no_hole(&b, 0, n), true);
        assert_eq!(has_no_hole(&b, 1, n), true);
    }

    #[test]
    fn test_has_no_hole_ng() {
        let n = 5;
        let b = vec![
            vec![0, 1, 1, 1, 0],
            vec![1, 0, 1, 1, 0],
            vec![1, 0, 0, 1, 0],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
        ];
        assert_eq!(has_no_hole(&b, 0, n), false);
        assert_eq!(has_no_hole(&b, 1, n), true);
    }
}
