use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
       n:usize,
       mut count: [usize;3],
       s:[String;n],
    }
    let all = count[0] + count[1] + count[2];
    let mut ans = vec![];
    for i in 0..n {
        let mut target = vec![];
        if &s[i] == "AB" {
            target.push((count[0], 0));
            target.push((count[1], 1));
        } else if &s[i] == "AC" {
            target.push((count[0], 0));
            target.push((count[2], 2));
        } else {
            target.push((count[1], 1));
            target.push((count[2], 2));
        }
        target.sort();
        if target[0].0 == 0 && target[1].0 == 0 {
            println!("No");
            return;
        }
        if target[0].0 == 1 && target[1].0 == 1 && all == 2 {
            if i == n - 1 {
                ans.push((b'A' + target[0].1 as u8) as char);
            } else {
                let mut next = vec![0; 3];
                next[target[0].1] += 1;
                next[target[1].1] += 1;
                if &s[i + 1] == "AB" {
                    next[0] += 1;
                    next[1] += 1;
                } else if &s[i + 1] == "AC" {
                    next[0] += 1;
                    next[2] += 1;
                } else {
                    next[1] += 1;
                    next[2] += 1;
                }
                if next[0] == 2 {
                    f(0, &mut count, target);
                    ans.push((b'A' + 0) as char);
                } else if next[1] == 2 {
                    f(1, &mut count, target);
                    ans.push((b'A' + 1) as char);
                } else {
                    f(2, &mut count, target);
                    ans.push((b'A' + 2) as char);
                }
            }
        } else {
            count[target[0].1] += 1;
            count[target[1].1] -= 1;
            ans.push((b'A' + target[0].1 as u8) as char);
        }
    }
    println!("Yes");
    for a in ans {
        println!("{}", a);
    }
}

fn f(a: usize, count: &mut Vec<usize>, target: Vec<(usize, usize)>) {
    count[a] += 1;
    for (_, i) in target {
        if a == i {
            continue;
        }
        count[i] -= 1;
    }
}
