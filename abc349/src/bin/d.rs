use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l:usize,
        r:usize,
    }
    let mut ans = vec![];
    dfs(l, r, 0, 1 << 60, &mut ans);
    println!("{}", ans.len());
    for (l, r) in ans {
        println!("{} {}", l, r);
    }
}

fn dfs(l: usize, r: usize, rl: usize, rr: usize, ans: &mut Vec<(usize, usize)>) {
    let rm = (rl + rr) / 2;
    if rl == l && rr == r {
        ans.push((rl, rr));
    } else if l < rm && rm < r {
        dfs(l, rm, rl, rm, ans);
        dfs(rm, r, rm, rr, ans);
    } else if r <= rm {
        dfs(l, r, rl, rm, ans);
    } else if rm <= l {
        dfs(l, r, rm, rr, ans);
    }
}
