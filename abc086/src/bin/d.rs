use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        xyc:[(usize,usize,char);n],
    }
    let mut b = vec![vec![0; k + 1]; k + 1];
    let mut w = vec![vec![0; k + 1]; k + 1];
    for (mut x, mut y, mut c) in xyc {
        x %= 2 * k;
        y %= 2 * k;
        if k <= x {
            x -= k;
            c = rev(c);
        }
        if k <= y {
            y -= k;
            c = rev(c);
        }
        if c == 'B' {
            b[x + 1][y + 1] += 1;
        } else {
            w[x + 1][y + 1] += 1;
        }
    }
    for i in 0..=k {
        for j in 0..k {
            b[i][j + 1] = b[i][j + 1] + b[i][j];
            w[i][j + 1] = w[i][j + 1] + w[i][j];
        }
    }
    for i in 0..k {
        for j in 0..=k {
            b[i + 1][j] = b[i + 1][j] + b[i][j];
            w[i + 1][j] = w[i + 1][j] + w[i][j];
        }
    }
    let mut ans = 0;
    for i in 0..=k {
        for j in 0..=k {
            let ww = w[i][k] + w[k][j] - 2 * w[i][j];
            let bb = b[i][k] + b[k][j] - 2 * b[i][j];
            ans = ans.max(ww + b[k][k] - bb).max(w[k][k] - ww + bb);
        }
    }
    println!("{}", ans);
}

fn rev(c: char) -> char {
    if c == 'B' {
        'W'
    } else {
        'B'
    }
}
