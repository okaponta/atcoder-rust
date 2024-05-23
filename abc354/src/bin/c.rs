use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        mut ac:[(usize,usize);n],
    }
    let mut aci = vec![];
    for (i, (a, c)) in ac.into_iter().enumerate() {
        aci.push((a, c, i));
    }
    aci.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    let mut max = 0;
    let mut ans = vec![];
    for (a, _, i) in aci.into_iter() {
        if a < max {
            continue;
        }
        max = max.max(a);
        ans.push(i + 1);
    }
    ans.sort();
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
