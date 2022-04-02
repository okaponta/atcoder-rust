use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n:usize,
        a:(Usize1,Usize1),
        b:(Usize1,Usize1),
        s:[Chars;n],
    }
    let mut able = vec![vec![true; n]; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                able[i][j] = false;
            }
        }
    }
    let mut dist = vec![vec![1 << 30; n]; n];
    dist[a.0][a.1] = 0;
    let mut prev = vec![(a.0, a.1, true), (a.0, a.1, false)];
    let mut count = 1;
    while !prev.is_empty() {
        let mut next = vec![];
        for point in prev {
            if !point.2 {
                let mut x = point.0;
                let mut y = point.1;
                while x + 1 < n && y + 1 < n {
                    x += 1;
                    y += 1;
                    if !able[x][y] {
                        break;
                    }
                    if dist[x][y] < count {
                        break;
                    }
                    if dist[x][y] > count {
                        dist[x][y] = count;
                        next.push((x, y, true));
                    }
                }
                x = point.0;
                y = point.1;
                while x > 0 && y > 0 {
                    x -= 1;
                    y -= 1;
                    if !able[x][y] {
                        break;
                    }
                    if dist[x][y] < count {
                        break;
                    }
                    if dist[x][y] > count {
                        dist[x][y] = count;
                        next.push((x, y, true));
                    }
                }
            } else {
                let mut x = point.0;
                let mut y = point.1;
                while x + 1 < n && y > 0 {
                    x += 1;
                    y -= 1;
                    if !able[x][y] {
                        break;
                    }
                    if dist[x][y] < count {
                        break;
                    }
                    if dist[x][y] > count {
                        dist[x][y] = count;
                        next.push((x, y, false));
                    }
                }
                x = point.0;
                y = point.1;
                while x > 0 && y + 1 < n {
                    x -= 1;
                    y += 1;
                    if !able[x][y] {
                        break;
                    }
                    if dist[x][y] < count {
                        break;
                    }
                    if dist[x][y] > count {
                        dist[x][y] = count;
                        next.push((x, y, false));
                    }
                }
            }
        }
        prev = next;
        count += 1;
    }
    let mut ans = dist[b.0][b.1];
    if ans == 1 << 30 {
        ans = -1;
    }
    println!("{}", ans);
}
