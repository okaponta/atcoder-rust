use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut ha:usize,
        mut wa:usize,
        mut a:[Chars;ha],
        mut hb:usize,
        mut wb:usize,
        mut b:[Chars;hb],
        hx:usize,
        wx:usize,
        x:[Chars;hx],
    }
    let (ha, wa, a) = trim_empty_rounds(ha, wa, a);
    let (hb, wb, b) = trim_empty_rounds(hb, wb, b);
    if hx < ha || hx < hb || wx < wa || wx < wb {
        println!("No");
        return;
    }
    for (i, j, k, l) in iproduct!(0..=hx - ha, 0..=wx - wa, 0..=hx - hb, 0..=wx - wb) {
        let mut ans = vec![vec!['.'; wx]; hx];
        for (m, n) in iproduct!(0..ha, 0..wa) {
            if a[m][n] == '#' {
                ans[i + m][j + n] = '#';
            }
        }
        for (m, n) in iproduct!(0..hb, 0..wb) {
            if b[m][n] == '#' {
                ans[k + m][l + n] = '#';
            }
        }
        if is_same(hx, wx, &ans, &x) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn trim_empty_rounds(
    mut h: usize,
    mut w: usize,
    mut a: Vec<Vec<char>>,
) -> (usize, usize, Vec<Vec<char>>) {
    while (0..w).into_iter().all(|j| a[0][j] == '.') {
        a.remove(0);
        h -= 1;
    }
    while (0..w).into_iter().rev().all(|j| a[h - 1][j] == '.') {
        a.remove(h - 1);
        h -= 1;
    }
    while (0..h).into_iter().all(|i| a[i][0] == '.') {
        for j in 0..h {
            a[j].remove(0);
        }
        w -= 1;
    }
    while (0..h).into_iter().rev().all(|i| a[i][w - 1] == '.') {
        for j in 0..h {
            a[j].remove(w - 1);
        }
        w -= 1;
    }
    (h, w, a)
}

fn is_same(h: usize, w: usize, a: &Vec<Vec<char>>, b: &Vec<Vec<char>>) -> bool {
    for i in 0..h {
        for j in 0..w {
            if a[i][j] != b[i][j] {
                return false;
            }
        }
    }
    true
}
