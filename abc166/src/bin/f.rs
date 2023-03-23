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
        let mut tmp = vec![0; 3];
        add_target(&s[i], &mut tmp);
        let mut t = get_index(&tmp, &count);
        if count[t[0]] == 0 && count[t[1]] == 0 {
            println!("No");
            return;
        }
        if count[t[0]] == 1 && count[t[1]] == 1 && all == 2 && i != n - 1 {
            add_target(&s[i + 1], &mut tmp);
            let inc = tmp.iter().enumerate().find(|(_, i)| **i == 2).unwrap().0;
            if t[0] != inc {
                t.swap(0, 1);
            }
        }
        count[t[0]] += 1;
        count[t[1]] -= 1;
        ans.push((b'A' + t[0] as u8) as char);
    }
    println!("Yes");
    for a in ans {
        println!("{}", a);
    }
}

fn add_target(s: &String, tmp: &mut Vec<usize>) {
    for i in 0..3 {
        if s.contains((b'A' + i as u8) as char) {
            tmp[i] += 1;
        }
    }
}

fn get_index(tmp: &Vec<usize>, count: &Vec<usize>) -> Vec<usize> {
    let mut res = vec![];
    for i in 0..3 {
        if tmp[i] != 0 {
            res.push(i);
        }
    }
    if count[res[1]] < count[res[0]] {
        res.swap(0, 1);
    }
    res
}
