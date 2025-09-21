#[rustfmt::skip]
use {itertools::*,proconio::*};

#[fastout]
fn main() {
    input! {t:usize}
    for _ in 0..t {
        case();
    }
}

fn case() {
    input! {n:usize, p:[usize;2usize.pow(n as u32)]}
    let mut v = vec![];
    for p in p {
        v.push(vec![p]);
    }
    for _ in 0..n {
        let mut new = vec![];
        for i in (0..v.len()).step_by(2) {
            if v[i + 1][0] < v[i][0] {
                v.swap(i, i + 1);
            }
            let mut tmp = vec![];
            for j in 0..v[i].len() {
                tmp.push(v[i][j]);
            }
            for j in 0..v[i].len() {
                tmp.push(v[i + 1][j]);
            }
            new.push(tmp);
        }
        v = new;
    }
    println!("{}", v[0].iter().join(" "));
}
