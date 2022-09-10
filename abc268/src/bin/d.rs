use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        s:[String;n],
        t:[String;m],
    }
    let mut set = HashSet::new();
    for t in t {
        set.insert(t);
    }
    let mut char_num = 0;
    for i in 0..n {
        char_num += s[i].len();
    }
    if 16 < char_num + n - 1 {
        println!("-1");
        return;
    }
    if n == 1 {
        if s[0].len() < 3 || 16 < s[0].len() {
            println!("-1");
            return;
        }
        if set.contains(&s[0]) {
            println!("-1");
        } else {
            println!("{}", s[0]);
        }
        return;
    }
    for perm in s.iter().permutations(n) {
        // _の数
        for i in 0..=(16 - char_num + 1 - n) {
            // _の位置
            for j in (0..n - 1).combinations_with_replacement(i) {
                let mut under_count = vec![1; n - 1];
                for k in j {
                    under_count[k] += 1;
                }
                let mut ans = "".to_string();
                for k in 0..n {
                    ans += perm[k];
                    if k == n - 1 {
                        break;
                    }
                    let underscore = match under_count[k] {
                        0 => "",
                        1 => "_",
                        2 => "__",
                        3 => "___",
                        4 => "____",
                        5 => "_____",
                        6 => "______",
                        7 => "_______",
                        8 => "________",
                        9 => "_________",
                        10 => "_________",
                        11 => "__________",
                        12 => "___________",
                        13 => "____________",
                        14 => "_____________",
                        15 => "______________",
                        16 => "_______________",
                        _ => "___",
                    };
                    ans += underscore;
                }
                if !set.contains(&ans) {
                    println!("{}", ans);
                    return;
                }
            }
        }
    }
    println!("-1");
}
