use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        x:Usize1,
    }
    let mut sp = vec![(1, 1)];
    for i in 0..n {
        sp.push((sp[i].0 * 2 + 3, sp[i].1 * 2 + 1));
    }
    println!("{}", f(n, x, &sp));
}

fn f(level: usize, k: usize, sp: &Vec<(usize, usize)>) -> usize {
    if level == 0 {
        return 1;
    }
    if k == 0 {
        return 0;
    }
    let mid = sp[level].0 / 2;
    if k < mid {
        f(level - 1, k - 1, sp)
    } else if k == mid {
        sp[level - 1].1 + 1
    } else {
        f(level - 1, k - sp[level].0 / 2 - 1, sp) + sp[level - 1].1 + 1
    }
}
