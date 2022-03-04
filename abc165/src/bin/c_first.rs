use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize,m:usize,q:usize,
       abcd:[(Usize1,Usize1,usize,usize);q],
    }
    let mut a = vec![1; n];
    let mut ans = 0;
    while a[0] != m {
        let mut point = 0;
        for (ai, bi, ci, di) in &abcd {
            if a[*bi] - a[*ai] == *ci {
                point += di;
            }
        }
        ans = ans.max(point);
        next(&mut a, n, m);
    }
    let mut point = 0;
    for (ai, bi, ci, di) in &abcd {
        if a[*bi] - a[*ai] == *ci {
            point += di;
        }
    }
    ans = ans.max(point);
    println!("{}", ans);
}

fn next(prev: &mut Vec<usize>, n: usize, m: usize) {
    let mut index = 100;
    for i in (0..n).rev() {
        if prev[i] != m {
            index = i;
            break;
        }
    }
    prev[index] += 1;
    for i in index..n {
        prev[i] = prev[index];
    }
}
