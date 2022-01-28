use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

const INF: usize = 1 << 60;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [usize; 2],
        d: [usize; 2],
        sdash: [Chars; h],
    }
    let mut v = vec![vec![INF; w + 4]; h + 4];
    let mut s = vec![vec!['#'; w + 4]; h + 4];
    for i in 0..h {
        for j in 0..w {
            s[i + 2][j + 2] = sdash[i][j];
        }
    }
    let mut q = VecDeque::new();

    q.push_back((c[0] + 1, c[1] + 1, 0));

    while q.len() > 0 {
        let (x, y, cost) = q.pop_front().unwrap();
        if v[x][y] <= cost {
            continue;
        }
        v[x][y] = cost;
        for i in -2..3 {
            for j in -2..3 {
                let fx = (x as isize + i) as usize;
                let fy = (y as isize + j) as usize;
                if v[fx][fy] == INF && s[fx][fy] == '.' {
                    if (i, j) == (1, 0)
                        || (i, j) == (0, 1)
                        || (i, j) == (-1, 0)
                        || (i, j) == (0, -1)
                    {
                        // 0-1BFS
                        // cost0の場合は前に追加
                        q.push_front((fx, fy, cost));
                    } else {
                        //cost1の場合は後ろに追加
                        q.push_back((fx, fy, cost + 1));
                    }
                }
            }
        }
    }
    if v[d[0] + 1][d[1] + 1] == INF {
        println!("-1");
    } else {
        println!("{}", v[d[0] + 1][d[1] + 1]);
    }
}
