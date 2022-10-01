use proconio::input;

fn main() {
    input! {
        n:usize,
        s:usize,
        ab:[(usize,usize);n],
    }
    let mut cur = vec!["".to_string(); s + 1];
    if ab[0].0 <= s {
        cur[ab[0].0] += "H";
    }
    if ab[0].1 <= s {
        cur[ab[0].1] += "T";
    }
    for i in 1..n {
        let mut next = vec!["".to_string(); s + 1];
        for j in 0..s {
            if cur[j] == "".to_string() {
                continue;
            } else {
                if j + ab[i].0 <= s {
                    next[j + ab[i].0] = cur[j].clone() + "H";
                }
                if j + ab[i].1 <= s {
                    next[j + ab[i].1] = cur[j].clone() + "T";
                }
            }
        }
        cur = next;
    }
    if cur[s] == "".to_string() {
        println!("No");
        return;
    } else {
        println!("Yes");
        println!("{}", cur[s]);
    }
}
