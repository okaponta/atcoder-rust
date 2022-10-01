use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
        abc:[(Usize1,Usize1,usize);m],
        e:[Usize1;k],
    }
    const INF: usize = 1 << 60;
    let mut distance = vec![INF; n];
    distance[0] = 0;
    for e in e {
        let from = abc[e].0;
        let to = abc[e].1;
        let dist = abc[e].2;
        if distance[from] == INF {
            continue;
        }
        distance[to] = distance[to].min(distance[from] + dist);
    }
    if distance[n - 1] == INF {
        println!("-1");
    } else {
        println!("{}", distance[n - 1]);
    }
}
