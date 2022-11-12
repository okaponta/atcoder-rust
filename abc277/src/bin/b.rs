use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        s:[String;n],
    }
    let mut set = HashSet::new();
    let ones = vec!['H', 'D', 'C', 'S'].into_iter().collect::<HashSet<_>>();
    let twos = vec![
        'A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K',
    ]
    .into_iter()
    .collect::<HashSet<_>>();
    for s in s {
        if set.contains(&s) {
            println!("No");
            return;
        }
        let one = s.chars().next().unwrap();
        if !ones.contains(&one) {
            println!("No");
            return;
        }
        let two = s.chars().nth(1).unwrap();
        if !twos.contains(&two) {
            println!("No");
            return;
        }
        set.insert(s);
    }
    println!("Yes");
}
