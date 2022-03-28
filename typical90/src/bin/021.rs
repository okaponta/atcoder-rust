use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut edge = vec![vec![]; n];
    let mut rev_edge = vec![vec![]; n];
    for (a, b) in ab {
        edge[a].push(b);
        rev_edge[b].push(a);
    }
    let mut used = vec![false; n];
    let mut num = vec![0; n];
    // num[i] -> i番目の番号がどの頂点か
    let mut count = 0;
    for i in 0..n {
        if !used[i] {
            count = dfs(i, count, &mut used, &mut num, &edge);
        }
    }
    // 以前の頂点 -> 新たな頂点のマッピング
    used = vec![false; n];
    let mut new_num = vec![0; n];
    let mut count = 0;
    let mut sizes = vec![];
    for i in (0..n).rev() {
        let target = num[i];
        if !used[target] {
            let size = rev_dfs(target, count, 0, &mut used, &mut new_num, &rev_edge);
            sizes.push(size);
            count += 1;
        }
    }
    let ans = sizes.iter().fold(0, |acc, x| acc + x * (x - 1) / 2);
    println!("{}", ans);
}

fn dfs(
    cur: usize,
    mut count: usize,
    used: &mut Vec<bool>,
    num: &mut Vec<usize>,
    edge: &Vec<Vec<usize>>,
) -> usize {
    used[cur] = true;
    for &next in edge[cur].iter() {
        if !used[next] {
            count = dfs(next, count, used, num, edge);
        }
    }
    num[count] = cur;
    count + 1
}

fn rev_dfs(
    cur: usize,
    count: usize,
    mut size: usize,
    used: &mut Vec<bool>,
    num: &mut Vec<usize>,
    edge: &Vec<Vec<usize>>,
) -> usize {
    used[cur] = true;
    for &next in edge[cur].iter() {
        if !used[next] {
            size = rev_dfs(next, count, size, used, num, edge);
        }
    }
    num[cur] = count;
    size + 1
}
