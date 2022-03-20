use std::{collections::HashSet, io::stdin};

fn main() {
    let n: usize = read().parse().unwrap();
    let mut set = HashSet::new();
    loop {
        for i in 1..=2 * n + 1 {
            if set.contains(&i) {
                continue;
            }
            set.insert(i);
            println!("{}", i);
            break;
        }
        let a: usize = read().parse().unwrap();
        if a == 0 {
            return;
        }
        set.insert(a);
    }
}

fn read() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    buf.split_whitespace().next().unwrap().to_string()
}
