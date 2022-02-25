use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
       m:usize,
       uv:[(Usize1,Usize1);m],
       p:[Usize1;8],
    }

    let mut edge = vec![vec![]; 9];
    for (u, v) in uv {
        edge[u].push(v);
        edge[v].push(u);
    }

    let goal = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
    let mut ans = vec![0; 9];
    for (i, p) in p.iter().enumerate() {
        ans[*p] = i + 1;
    }

    let mut map: HashMap<Vec<usize>, usize> = HashMap::new();

    let mut prev_tops = vec![goal];
    let mut count = 0;

    while !prev_tops.is_empty() {
        let mut next_tops = vec![];
        for top in prev_tops {
            if map.contains_key(&top) {
                continue;
            }
            map.insert(top.clone(), count);

            let space_index = top.iter().enumerate().find(|(_, &v)| v == 0).unwrap().0;
            for next in &edge[space_index] {
                let mut n_top = top.clone();
                n_top.swap(space_index, *next);
                next_tops.push(n_top);
            }
        }
        prev_tops = next_tops;
        count += 1;
    }

    match map.get(&ans) {
        Some(res) => println!("{}", res),
        None => println!("-1"),
    }
}
