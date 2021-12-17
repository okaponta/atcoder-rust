use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize,q:usize,
       ab: [(Usize1,Usize1);n-1],
       cd: [(Usize1,Usize1);q]
    }
    let mut road = vec![vec![]; n];
    for (a, b) in ab {
        road[a].push(b);
        road[b].push(a);
    }

    let mut depth = vec![-1; n];
    let mut s = vec![];
    depth[0] = 0;
    s.push(0);

    while let Some(x) = s.pop() {
        for &y in &road[x] {
            if depth[y] != -1 {
                continue;
            }
            depth[y] = depth[x] + 1;
            s.push(y);
        }
    }

    for (c, d) in cd {
        println!(
            "{}",
            if (depth[c] - depth[d]) % 2 == 0 {
                "Town"
            } else {
                "Road"
            }
        );
    }
}
