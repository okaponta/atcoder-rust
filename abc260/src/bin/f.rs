use proconio::{input, marker::Usize1};

fn main() {
    input! {
        s:usize,
        t:usize,
        m:usize,
        uv:[(Usize1,Usize1);m],
    }
    let mut edges = vec![vec![]; s];
    for (u, v) in uv {
        edges[u].push(v);
    }
    let mut map = vec![vec![!0; t]; t];
    for i in 0..s {
        let n = edges[i].len();
        for j in 0..n {
            for k in j + 1..n {
                let (j, k) = (edges[i][j].min(edges[i][k]), edges[i][j].max(edges[i][k]));
                if map[j - s][k - s] != !0 {
                    println!("{} {} {} {}", i + 1, j + 1, k + 1, map[j - s][k - s] + 1);
                    return;
                } else {
                    map[j - s][k - s] = i;
                }
            }
        }
    }
    println!("-1");
}
