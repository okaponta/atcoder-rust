use proconio::input;

fn main() {
    input! {
        l:usize,
        n1:usize,
        n2:usize,
        vl1:[(i64,usize);n1],
        vl2:[(i64,usize);n2],
    }
    let mut vls = vec![(0, 0)];
    let mut vls1 = vec![(0, 0)];
    for i in 0..n1 {
        vls1.push((vl1[i].0, vls1[i].1 + vl1[i].1));
        vls.push((0, vls1[i].1 + vl1[i].1));
    }
    let mut vls2 = vec![(0, 0)];
    for i in 0..n2 {
        vls2.push((vl2[i].0, vls2[i].1 + vl2[i].1));
        vls.push((0, vls2[i].1 + vl2[i].1));
    }
    vls.sort();
    vls.dedup();
    vls.push((1, l + 1));
    let mut index = 0;
    for (val, i) in vls1 {
        while vls[index].1 <= i {
            vls[index].0 += val;
            index += 1;
        }
    }
    let mut index = 0;
    for (val, i) in vls2 {
        while vls[index].1 <= i {
            vls[index].0 -= val;
            index += 1;
        }
    }
    let mut ans = 0;
    for i in 1..vls.len() {
        if vls[i].0 == 0 {
            ans += vls[i].1 - vls[i - 1].1;
        }
    }
    println!("{}", ans);
}
