use itertools::Itertools;
use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        l:Usize1,
        r:Usize1,
    }
    let mut dict = vec![0; n];
    for i in 1..n {
        dict[i] = dict[i - 1] + n - i;
    }
    let from = dict.upper_bound(&l) - 1;
    let to = dict.upper_bound(&r) - 1;
    let mut ans = (1..=n).collect_vec();

    let start = l + from + 1 - dict[from];
    let end = r + to + 1 - dict[to];

    if from == to {
        loopswap(&mut ans, from, start, end);
        println!("{}", ans.iter().join(" "));
        return;
    }

    loopswap(&mut ans, from, start, n - 1);

    ans[from + 1..].rotate_right(to - from - 1);
    ans[from + 1..to].reverse();

    loopswap(&mut ans, to, to + 1, end);
    println!("{}", ans.iter().join(" "));
}

fn loopswap(ans: &mut Vec<usize>, first: usize, start: usize, end: usize) {
    for i in start..=end {
        ans.swap(first, i);
    }
}
