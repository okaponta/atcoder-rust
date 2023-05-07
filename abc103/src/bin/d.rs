use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        mut ab:[(Usize1,Usize1);m],
    }
    ab.sort_by(|a, b| a.1.cmp(&b.1));
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[b].push(a);
    }
    let mut min = 0;
    let mut ans = 0;
    for i in 1..n {
        for &prev in &g[i] {
            if min <= prev {
                ans += 1;
                min = i;
                break;
            }
        }
    }
    println!("{}", ans);
}
