use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        mut xy:[(usize,usize);n],
        s:Chars,
    }
    let mut xys = vec![];
    for i in 0..n {
        xys.push((xy[i].0, xy[i].1, s[i]));
    }
    xys.sort_by_key(|x| x.1);
    let mut prev = 2 << 60;
    let mut right = 2 << 60;
    let mut left = 0;
    for (x, y, s) in xys {
        if prev != y {
            if right < left {
                println!("Yes");
                return;
            }
            right = 2 << 60;
            left = 0;
        }
        if s == 'R' {
            right = right.min(x);
        } else {
            left = left.max(x);
        }
        prev = y;
    }
    if right < left {
        println!("Yes");
        return;
    }
    println!("No");
}
