use std::collections::HashSet;
use text_io::read;

fn main() {
    let n: usize = read!();

    let mut set = HashSet::new();
    let mut i = 1;

    while i <= 2 * n + 1 {
        while set.contains(&i) {
            i += 1;
        }
        set.insert(i);
        println!("{}", i);
        let x: usize = read!();
        set.insert(x);

        if x == 0 {
            return;
        }
    }
}
