use proconio::input;

fn main() {
    input! {
        n:usize,
        p:i64,
        k:usize,
        a:[[i64;n];n],
    }
    let mut lower = 0;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if k <= cnt(n, med, p, &a) {
            lower = med;
        } else {
            upper = med;
        }
    }
    let max = lower;
    if cnt(n, max, p, &a) != k {
        println!("0");
        return;
    }
    let mut lower = 0;
    let mut upper = max;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if k == cnt(n, med, p, &a) {
            upper = med;
        } else {
            lower = med;
        }
    }
    if 1 << 40 < max {
        println!("Infinity");
        return;
    }
    println!("{}", max - lower);
}

fn cnt(n: usize, x: i64, p: i64, a: &Vec<Vec<i64>>) -> usize {
    let mut g = vec![vec![]; n];
    for i in 0..n {
        for j in i..n {
            g[i].push((j, if a[i][j] == -1 { x } else { a[i][j] }));
            g[j].push((i, if a[i][j] == -1 { x } else { a[i][j] }));
        }
    }
    let w = WarshallFloyd::new(n, &g);
    let mut count = 0;
    for i in 0..n {
        for j in 0..i {
            if w.distance[i][j] <= p {
                count += 1;
            }
        }
    }
    count
}

pub struct WarshallFloyd {
    distance: Vec<Vec<i64>>,
}

impl WarshallFloyd {
    // n:usize 頂点の数
    // edges: Vec<Vec<(usize,i64)>> edges[i] = [(2,3), (3,-1), (To,重み)]
    pub fn new(n: usize, edges: &Vec<Vec<(usize, i64)>>) -> Self {
        let mut distance = vec![vec![1 << 60; n]; n];
        for i in 0..n {
            for &(j, c) in &edges[i] {
                distance[i][j] = c;
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    distance[i][j] = distance[i][j].min(distance[i][k] + distance[k][j]);
                    // 普通に足す場合は足してこの場合はmaxをとる問題だった(こんなパターンもあるよ紹介)
                    // distance[i][j] = distance[i][j].min(distance[i][k].max(distance[k][j]));
                }
            }
        }
        Self { distance }
    }
}
