use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        w:usize,
        wv:[(usize,usize);n],
    }
    let mut map: HashMap<usize, usize> = HashMap::from([(0, 0)]);
    for (wi, vi) in wv {
        let mut nmap = map.clone();
        for (k, v) in map {
            if k + wi <= w {
                let bef = nmap.get(&(k + wi)).unwrap_or(&0);
                nmap.insert(k + wi, (v + vi).max(*bef));
            }
        }
        map = nmap;
    }
    println!("{}", map.into_iter().map(|(_, v)| v).max().unwrap());
}
