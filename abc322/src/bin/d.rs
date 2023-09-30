use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut p:[[Chars;4];3],
    }
    p[0] = upleft(&p[0]);
    for (p1i, p1j, p2i, p2j, p3i, p3j) in iproduct!(0..4, 0..4, 0..4, 0..4, 0..4, 0..4) {
        let pij = vec![(p1i, p1j), (p2i, p2j), (p3i, p3j)];
        for _ in 0..4 {
            for _ in 0..4 {
                if check(&p, &pij) {
                    println!("Yes");
                    return;
                }
                p[1] = rotate_2d_vector(&p[1]);
                p[1] = upleft(&p[1]);
            }
            p[2] = rotate_2d_vector(&p[2]);
            p[2] = upleft(&p[2]);
        }
    }
    println!("No");
}

fn check(p: &Vec<Vec<Vec<char>>>, pij: &Vec<(usize, usize)>) -> bool {
    let mut used = vec![vec![false; 4]; 4];
    for (k, i, j) in iproduct!(0..3, 0..4, 0..4) {
        if p[k][i][j] == '#' {
            let ni = pij[k].0 + i;
            let nj = pij[k].1 + j;
            if 3 < ni || 3 < nj || used[ni][nj] {
                return false;
            }
            used[ni][nj] = true;
        }
    }
    (0..4)
        .into_iter()
        .all(|i| (0..4).into_iter().all(|j| used[i][j]))
}

fn upleft(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let w = v.len();
    let h = v[0].len();
    let mut new_v = vec![vec!['.'; w]; h];
    let mut upshift = 0;
    let mut leftshift = 0;
    for i in 0..h {
        if (0..w).into_iter().all(|j| v[i][j] == '.') {
            upshift += 1;
        } else {
            break;
        }
    }
    for j in 0..w {
        if (0..h).into_iter().all(|i| v[i][j] == '.') {
            leftshift += 1;
        } else {
            break;
        }
    }
    for i in upshift..h {
        for j in leftshift..w {
            new_v[i - upshift][j - leftshift] = v[i][j];
        }
    }
    return new_v;
}

fn rotate_2d_vector(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_v = vec![vec!['.'; v.len()]; v[0].len()];
    for i in 0..v.len() {
        for j in 0..v[0].len() {
            new_v[j][v.len() - 1 - i] = v[i][j];
        }
    }
    return new_v;
}
