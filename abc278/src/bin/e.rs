use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        hh:usize,
        ww:usize,
        n:usize,
        h:usize,
        w:usize,
        a:[[usize;ww];hh],
    }
    let mut num = vec![0; n + 1];
    for i in 0..hh {
        for j in 0..ww {
            num[a[i][j]] += 1;
        }
    }
    for i in 0..=hh - h {
        let mut tmp = vec![0; n + 1];
        for ii in i..h + i {
            for jj in 0..w {
                tmp[a[ii][jj]] += 1;
            }
        }
        let mut ans = vec![countnum(&num, &tmp, n)];
        for j in 0..ww - w {
            for ii in i..h + i {
                tmp[a[ii][j]] -= 1;
                tmp[a[ii][j + w]] += 1;
            }
            ans.push(countnum(&num, &tmp, n));
        }
        println!("{}", ans.iter().join(" "));
    }
}

fn countnum(all: &Vec<usize>, tmp: &Vec<usize>, n: usize) -> usize {
    let mut ans = 0;
    for i in 1..=n {
        if tmp[i] < all[i] {
            ans += 1;
        }
    }
    ans
}
