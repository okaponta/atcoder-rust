use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
    }
    let mut testinomy = vec![vec![]; n];
    for i in 0..n {
        input! {
            a: usize,
            xy: [(Usize1,u8);a],
        }
        let mut honest = vec![];
        for i in 0..a {
            honest.push((xy[i].0, xy[i].1 == 1));
        }
        testinomy[i] = honest;
    }

    let mut ans = 0;
    for i in 0usize..1 << n {
        let mut honest = vec![false; n];
        for j in 0..n {
            if i >> j & 1 == 1 {
                honest[j] = true;
            }
        }
        let mut ok = true;
        for j in 0..n {
            if !honest[j] {
                continue;
            }
            let is_ok = testinomy[j]
                .iter()
                .all(|&(person, is_honest)| honest[person] == is_honest);
            if !is_ok {
                ok = false;
            }
        }
        if ok {
            ans = ans.max(i.count_ones());
        }
    }
    println!("{}", ans);
}
