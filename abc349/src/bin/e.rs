use proconio::input;

fn main() {
    input! {
        a:[[i64;3];3],
    }
    let mut b = vec![];
    for i in 0..3 {
        for j in 0..3 {
            b.push(a[i][j]);
        }
    }
    let mut used = vec![0; 9];
    let c = dfs(0, 0, 0, &mut used, &b);
    if c == 'T' {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}

fn dfs(turn: usize, tak: i64, aok: i64, used: &mut Vec<u8>, b: &Vec<i64>) -> char {
    if turn == 9 {
        if aok < tak {
            return 'T';
        } else {
            return 'A';
        }
    }
    if check(&used, 1) {
        return 'T';
    }
    if check(&used, 2) {
        return 'A';
    }
    let mut ret = vec![];
    for i in 0..9 {
        if used[i] != 0 {
            continue;
        }
        if turn % 2 == 0 {
            used[i] = 1;
            ret.push(dfs(turn + 1, tak + b[i], aok, used, b));
            used[i] = 0;
        } else {
            used[i] = 2;
            ret.push(dfs(turn + 1, tak, aok + b[i], used, b));
            used[i] = 0;
        }
    }
    if ret.iter().all(|c| c == &'A') {
        return 'A';
    }
    if ret.iter().all(|c| c == &'T') {
        return 'T';
    }
    if turn % 2 == 0 {
        return 'T';
    } else {
        return 'A';
    }
}

fn check(used: &Vec<u8>, i: u8) -> bool {
    if used[0] == i && used[1] == i && used[2] == i {
        return true;
    }
    if used[3] == i && used[4] == i && used[5] == i {
        return true;
    }
    if used[6] == i && used[7] == i && used[8] == i {
        return true;
    }
    if used[0] == i && used[3] == i && used[6] == i {
        return true;
    }
    if used[1] == i && used[4] == i && used[7] == i {
        return true;
    }
    if used[2] == i && used[5] == i && used[8] == i {
        return true;
    }
    if used[0] == i && used[4] == i && used[8] == i {
        return true;
    }
    if used[2] == i && used[4] == i && used[6] == i {
        return true;
    }
    false
}
