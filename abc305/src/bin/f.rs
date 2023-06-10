use std::io::{stdin, BufReader};

use proconio::{input, marker::Usize1, source::line::LineSource};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n:usize,
        _:usize,
    }
    let mut path = vec![0];
    let mut used = vec![false; n];
    used[0] = true;
    loop {
        input! {
            from &mut source,
            k:usize,
            v:[Usize1;k],
        }
        if v.contains(&(n - 1)) {
            println!("{}", n);
            input! {
                from &mut source,
                _:String,
            }
            return;
        }
        let mut flg = false;
        for &next in &v {
            if used[next] {
                continue;
            }
            flg = true;
            used[next] = true;
            path.push(next);
            println!("{}", next + 1);
            break;
        }
        if !flg {
            // 戻るしかない
            path.pop();
            println!("{}", path[path.len() - 1] + 1);
        }
    }
}
