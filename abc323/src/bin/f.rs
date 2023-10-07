use proconio::input;

fn main() {
    input! {
        p:(i64,i64),
        start:(i64,i64),
        goal:(i64,i64),
    }
    // x -> y
    let mut ans_a = 0;
    let mut cur_a = p;
    ans_a += mov_x(p, start, (goal.0, start.1), &mut cur_a);
    ans_a += mov_y(cur_a, (goal.0, start.1), goal, &mut cur_a);
    // y -> x
    let mut ans_b = 0;
    let mut cur_b = p;
    ans_b += mov_y(p, start, (start.0, goal.1), &mut cur_b);
    ans_b += mov_x(cur_b, (start.0, goal.1), goal, &mut cur_b);
    println!("{}", ans_a.min(ans_b));
}

fn mov_x(p: (i64, i64), start: (i64, i64), goal: (i64, i64), cur: &mut (i64, i64)) -> i64 {
    let mut res = 0;
    if start.0 < goal.0 {
        res += mov(p, (start.0 - 1, start.1), start);
        cur.0 = goal.0 - 1;
        cur.1 = start.1;
    }
    if goal.0 < start.0 {
        res += mov(p, (start.0 + 1, start.1), start);
        cur.0 = goal.0 + 1;
        cur.1 = start.1;
    }
    res += (start.0 - goal.0).abs();
    res
}

fn mov_y(p: (i64, i64), start: (i64, i64), goal: (i64, i64), cur: &mut (i64, i64)) -> i64 {
    let mut res = 0;
    if start.1 < goal.1 {
        res += mov(p, (start.0, start.1 - 1), start);
        cur.0 = start.0;
        cur.1 = goal.1 - 1;
    }
    if goal.1 < start.1 {
        res += mov(p, (start.0, start.1 + 1), start);
        cur.0 = start.0;
        cur.1 = goal.1 + 1;
    }
    res += (start.1 - goal.1).abs();
    res
}

fn mov(p: (i64, i64), q: (i64, i64), ng: (i64, i64)) -> i64 {
    if p == q {
        return 0;
    }
    if p.0 == q.0 && ((p.1 < ng.1 && ng.1 < q.1) || q.1 < ng.1 && ng.1 < p.1) {
        return (p.0 - q.0).abs() + (p.1 - q.1).abs() + 2;
    }
    if p.1 == q.1 && ((p.0 < ng.0 && ng.0 < q.0) || q.0 < ng.0 && ng.0 < p.0) {
        return (p.0 - q.0).abs() + (p.1 - q.1).abs() + 2;
    }
    (p.0 - q.0).abs() + (p.1 - q.1).abs()
}
