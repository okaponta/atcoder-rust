use proconio::input;

fn main() {
    input! {
        _:usize,
        n:usize,
        py:[(usize,usize);n],
    }
    let mut pyi = vec![];
    for i in 0..n {
        pyi.push((py[i].0, py[i].1, i));
    }
    pyi.sort();
    pyi.insert(0, (0, 0, 0));
    let mut ans = vec![(0, 0); n];
    let mut idx = 1;
    for i in 1..=n {
        if pyi[i - 1].0 != pyi[i].0 {
            idx = 1;
        }
        ans[pyi[i].2] = (pyi[i].0, idx);
        idx += 1;
    }
    for i in 0..n {
        println!("{:<06}{:<06}", ans[i].0, ans[i].1);
    }
}
