use proconio::input;

fn main() {
    input! {
        n:usize,
        xyp:[(i64,i64,i64);n],
    }
    let mut edges = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            let dist = (xyp[i].0 - xyp[j].0).abs() + (xyp[i].1 - xyp[j].1).abs();
            edges[i].push((j, (dist + xyp[i].2 - 1) / xyp[i].2));
        }
    }
    let wf = WarshallFloyd::new(n, &edges);
    let mut ans = 1 << 60;
    for i in 0..n {
        let mut temp = 0;
        for j in 0..n {
            temp = temp.max(wf.distance[i][j]);
        }
        ans = ans.min(temp);
    }
    println!("{}", ans);
}

pub struct WarshallFloyd {
    distance: Vec<Vec<i64>>,
}

impl WarshallFloyd {
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
                    distance[i][j] = distance[i][j].min(distance[i][k].max(distance[k][j]));
                }
            }
        }
        Self { distance }
    }
}
