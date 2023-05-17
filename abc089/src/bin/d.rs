use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        d:usize,
        a:[[Usize1;w];h],
        q:usize,
        lr:[(Usize1,Usize1);q],
    }
    let mut xy = vec![(0, 0); h * w];
    for i in 0..h {
        for j in 0..w {
            xy[a[i][j]] = (i, j);
        }
    }
    let mut s = vec![0; h * w];
    for i in 0..d {
        for j in (i..h * w).step_by(d) {
            if j < d {
                s[j] = 0;
            } else {
                s[j] = s[j - d] + abs(xy[j], xy[j - d]);
            }
        }
    }
    for (l, r) in lr {
        println!("{}", s[r] - s[l]);
    }
}

fn abs(a: (usize, usize), b: (usize, usize)) -> usize {
    let mut res = 0;
    if a.0 < b.0 {
        res += b.0 - a.0;
    } else {
        res += a.0 - b.0;
    }
    if a.1 < b.1 {
        res += b.1 - a.1;
    } else {
        res += a.1 - b.1;
    }
    res
}
