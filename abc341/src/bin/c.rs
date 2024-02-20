use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        n:usize,
        t:Chars,
        s:[Chars;h],
    }
    let mut ans = 0;
    for i in 0..h {
        'a: for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            let mut tmp = (i, j);
            for k in 0..n {
                let (di, dj) = dir_to_int(t[k]);
                let ni = tmp.0.wrapping_add(di);
                let nj = tmp.1.wrapping_add(dj);
                if h <= ni || w <= nj || s[ni][nj] == '#' {
                    continue 'a;
                }
                tmp = (ni, nj);
            }
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn dir_to_int(c: char) -> (usize, usize) {
    match c {
        'U' => (!0, 0),
        'D' => (1, 0),
        'L' => (0, !0),
        'R' => (0, 1),
        _ => panic!(),
    }
}
