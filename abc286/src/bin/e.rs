use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[usize;n],
        s:[Chars;n],
        q:usize,
        uv:[(Usize1,Usize1);q],
    }
    let mut edges = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'Y' {
                edges[i].push((j, 1, a[j]));
            }
        }
    }
    let wf = WarshallFloyd::new(n, &edges);
    for (u, v) in uv {
        if 1 << 59 < wf.distance[u][v].0 {
            println!("Impossible");
            continue;
        }
        println!("{} {}", wf.distance[u][v].0, wf.distance[u][v].1 + a[u]);
    }
}

pub struct WarshallFloyd {
    distance: Vec<Vec<(usize, usize)>>,
}

impl WarshallFloyd {
    pub fn new(n: usize, edges: &Vec<Vec<(usize, usize, usize)>>) -> Self {
        let mut distance = vec![vec![(1 << 60, 0); n]; n];
        for i in 0..n {
            for &(j, c, p) in &edges[i] {
                distance[i][j] = (c, p);
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    // distance[i][j] = distance[i][j].min(distance[i][k].max(distance[k][j]));
                    distance[i][j] =
                        Self::mn(distance[i][j], Self::plus(distance[i][k], distance[k][j]));
                }
            }
        }
        Self { distance }
    }

    fn mn(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
        if a.0 < b.0 {
            return a;
        }
        if b.0 < a.0 {
            return b;
        }
        if b.1 < a.1 {
            return a;
        }
        b
    }

    fn plus(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
        (a.0 + b.0, a.1 + b.1)
    }
}
