use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        ab:[(Usize1,Usize1);n-1],
    }
    let mut edge = vec![vec![]; n];
    for i in 0..n - 1 {
        edge[ab[i].0].push((ab[i].1, i));
    }
    let mut ans = vec![0; n - 1];
    dfs(0, 0, &edge, &mut ans);
    println!("{}", ans.iter().max().unwrap());
    ans.iter().for_each(|x| println!("{}", x));
}

fn dfs(cur: usize, used: usize, edge: &Vec<Vec<(usize, usize)>>, ans: &mut Vec<usize>) {
    let mut c = 1;
    for &(next, i) in &edge[cur] {
        if c == used {
            c += 1;
        }
        ans[i] = c;
        dfs(next, c, edge, ans);
        c += 1;
    }
}
