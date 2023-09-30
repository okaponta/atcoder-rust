use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        p:[[Chars;4];3],
    }
    'a: for (p1i, p1j, p2i, p2j, p3i, p3j, x, y) in
        iproduct!(0..8, 0..8, 0..8, 0..8, 0..8, 0..8, 0..4, 0..4)
    {
        let mut target = vec![vec![0; 12]; 12];
        let pij = vec![(p1i, p1j), (p2i, p2j), (p3i, p3j)];
        for (n, i, j) in iproduct!(0..3, 0..4, 0..4) {
            let x = if n == 0 {
                0
            } else if n == 1 {
                x
            } else {
                y
            };
            if p[n][i][j] == '#' {
                let (ni, nj) = next(pij[n].0, pij[n].1, i, j, x);
                if ni < 4 || 7 < ni || nj < 4 || 7 < nj || target[ni][nj] == 1 {
                    continue 'a;
                }
                target[ni][nj] += 1;
            }
        }
        for (i, j) in iproduct!(4..8, 4..8) {
            if target[i][j] != 1 {
                continue 'a;
            }
        }
        println!("Yes");
        return;
    }
    println!("No");
}

fn next(pi: usize, pj: usize, i: usize, j: usize, x: usize) -> (usize, usize) {
    if x == 0 {
        return (pi + i, pj + j);
    }
    if x == 1 {
        return (pj + 3 - j, pi + i);
    }
    if x == 2 {
        return (pi + 3 - i, pj + 3 - j);
    }
    return (pj + j, pi + 3 - i);
}
