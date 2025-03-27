#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input_interactive! {
        n:usize,
        uv:[(Usize1,Usize1);n-1],
    }
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    let wf = WarshallFloyd::new(n, &g);
    let mut set = BTreeSet::new();
    for i in 0..n {
        for j in 0..i {
            if 1 < wf.distance[i][j] && wf.distance[i][j] % 2 == 1 {
                set.insert((j, i));
            }
        }
    }
    if set.len() % 2 == 0 {
        println!("Second");
    } else {
        println!("First");
        let p = set.pop_first().unwrap();
        println!("{} {}", p.0 + 1, p.1 + 1);
    }
    loop {
        input_interactive! {
            i:i64,
            j:i64,
        };
        if i == -1 && j == -1 {
            break;
        }
        set.remove(&(i as usize - 1, j as usize - 1));
        let p = set.pop_first().unwrap();
        println!("{} {}", p.0 + 1, p.1 + 1);
    }
}

pub struct WarshallFloyd {
    distance: Vec<Vec<usize>>,
}

impl WarshallFloyd {
    // n:usize 頂点の数
    // edges: Vec<Vec<(usize,i64)>> edges[i] = [(2,3), (3,-1), (To,重み)]
    pub fn new(n: usize, edges: &Vec<Vec<usize>>) -> Self {
        let mut distance = vec![vec![1 << 60; n]; n];
        for i in 0..n {
            distance[i][i] = 0;
            for &j in &edges[i] {
                distance[i][j] = 1;
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    distance[i][j] = distance[i][j].min(distance[i][k] + distance[k][j]);
                }
            }
        }
        Self { distance }
    }
}
