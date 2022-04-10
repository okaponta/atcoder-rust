use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        st:[(String,String);n],
    }
    let mut once = HashSet::new();
    let mut twice = HashSet::new();
    for (s, t) in st.clone() {
        if s == t {
            if once.contains(&s) {
                twice.insert(s);
            } else {
                once.insert(s);
            }
            continue;
        }
        if once.contains(&s) {
            twice.insert(s);
        } else {
            once.insert(s);
        }
        if once.contains(&t) {
            twice.insert(t);
        } else {
            once.insert(t);
        }
    }
    for (s, t) in st {
        if s == t {
            if twice.contains(&s) {
                println!("No");
                return;
            }
        }
        if twice.contains(&s) && twice.contains(&t) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
