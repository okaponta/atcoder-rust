use proconio::input;

fn main() {
    input! {
       _:usize,
       m:usize,
    }
    if m == 1 {
        println!("1 2");
        return;
    }
    let mut l1 = 1;
    let mut r1 = 1 + m;
    let mut l2 = r1 + 1;
    let mut r2 = l2 + m - 1;
    let mut ans = vec![];
    while l1 < r1 {
        ans.push((l1, r1));
        l1 += 1;
        r1 -= 1;
    }
    while l2 < r2 {
        ans.push((l2, r2));
        l2 += 1;
        r2 -= 1;
    }
    for (l, r) in ans {
        println!("{} {}", l, r);
    }
}
