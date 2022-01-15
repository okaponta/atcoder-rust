use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
       a:i32,n:i32
    }
    if n == 1 {
        println!("{}", 0);
        return;
    }
    let mut count = 1;
    let mut set = HashSet::new();
    set.insert(1);
    let mut prev = vec![1];

    while prev.len() > 0 {
        let mut next = vec![];
        for target in prev {
            let cand1 = target * a;
            if cand1 == n {
                println!("{}", count);
                return;
            }
            if cand1 < 1000000 && !set.contains(&cand1) {
                next.push(cand1);
                set.insert(cand1);
            }

            if target > 9 && target % 10 != 0 {
                let d = target % 10;
                let mut str = (target / 10).to_string();
                str.insert(0, std::char::from_digit(d as u32, 10).unwrap());
                let cand2 = str.parse::<i32>().unwrap();
                if cand2 == n {
                    println!("{}", count);
                    return;
                }
                if cand2 < 1000000 && !set.contains(&cand2) {
                    next.push(cand2);
                    set.insert(cand2);
                }
            }
        }
        count += 1;
        prev = next;
    }
    println!("{}", -1);
}
