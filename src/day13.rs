use std::{
    cmp::{max, min},
    io::{self, Read, Write},
};

fn main() {
    let mut stdout = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf).unwrap();

    //let stdin = buf.lines();
    //solve1(&mut stdin.clone(), &mut stdout);

    solve2(buf, &mut stdout);
}

fn solve<'a>(stdin: &mut impl Iterator<Item = &'a str>) -> usize {
    let mut hands = parse(stdin).unwrap();
    let n = normalize(&mut hands);

    let mut res = 0;

    let mut rounds = 0;

    loop {
        rounds += 1;

        let still: Vec<isize> = hands
            .iter()
            .filter(|&(_, moves)| *moves == 0)
            .map(|&((pos, _), _)| pos)
            .collect();

        let moving: Vec<I> = hands
            .iter()
            .filter(|&(_, moves)| *moves > 0)
            .map(|&(i, _)| i)
            .collect();

        if still.len() == hands.len() {
            break;
        }

        // check each hand
        for i in 0..moving.len() {
            let hi = moving[i];

            res += still.iter().filter(|&&p| contains_i(hi, p, n)).count();

            // check if it'll intersect any other hand (either in process,
            // or by landing at the same place as other hand)
            for j in i + 1..moving.len() {
                let hj = moving[j];

                if intersect_i(hi, hj, n) {
                    res += 1;
                }
            }
        }

        // move the hands
        for ((pos, delta), moves) in &mut hands {
            if *moves > 0 {
                *pos = move_i((*pos, *delta), n);
                *moves -= 1;
            }
        }
    }

    res
}

fn solve2<'a, W>(stdin: String, stdout: &mut W)
where
    W: Write,
{
    let mut res = 0;
    for block in stdin.split("\n\n") {
        res += solve(&mut block.lines());
    }
    writeln!(stdout, "{res}").unwrap();
}

// hand's current position and delta
type I = (isize, isize);

fn move_i(i: I, n: isize) -> isize {
    let (p, d) = i;
    (p + d).rem_euclid(n)
}

fn interval_i(i: I, n: isize) -> (I, bool) {
    let (l, d) = i;
    let r = move_i(i, n);

    let wraparound = (dir_i(i) == 1 && r < l) || (dir_i(i) == -1 && l < r);

    if !wraparound {
        if dir_i(i) == 1 {
            return ((l, r), false);
        }
        return ((r, l), false);
    }

    if dir_i(i) == -1 {
        assert!(l < r);
        return ((r, r + d.abs()), true);
    }

    assert!(r < l);
    ((l, l + d), true)
}

fn intersect_i(i1: I, i2: I, n: isize) -> bool {
    if move_i(i1, n) == move_i(i2, n) {
        return i1.0 != i2.0;
    }

    let ((l1, r1), _) = interval_i(i1, n);
    let ((l2, r2), _) = interval_i(i2, n);

    if dir_i(i1) == dir_i(i2) {
        let contains = |(a, b): I, (c, d): I| a < c && d < b;
        return contains((l1, r1), (l2, r2))
            || contains((l1, r1), (l2 + n, r2 + n))
            || contains((l2, r2), (l1, r1))
            || contains((l2, r2), (l1 + n, r1 + n));
    }

    let overlaps = |(a, b): I, (c, d): I| max(a, c) < min(b, d);
    overlaps((l1, r1), (l2, r2))
        || overlaps((l1, r1), (l2 + n, r2 + n))
        || overlaps((l1 + n, r1 + n), (l2, r2))
}

fn contains_i(i: I, p: isize, n: isize) -> bool {
    let ((l, r), w) = interval_i(i, n);

    if i.0 == p {
        return false;
    }

    if move_i(i, n) == p {
        return true;
    }

    if !w {
        return (l + 1..r).contains(&p);
    }

    !(min(i.0, move_i(i, n))..=max(i.0, move_i(i, n))).contains(&p)
}

fn dir_i(i: I) -> isize {
    i.1.signum()
}

// delta, pos, moves left
type H = (I, usize);

fn parse<'a>(stdin: &mut impl Iterator<Item = &'a str>) -> Option<Vec<H>> {
    let mut hands = stdin.next()?.strip_prefix("hands ")?.split(':');
    let dirs = stdin.next()?.strip_prefix("direction ")?.split(':');
    let mut currs = stdin.next()?.strip_prefix("current ")?.split(':');
    let mut moves = stdin.next()?.strip_prefix("move ")?.split(':');

    let mut res = Vec::new();

    for d in dirs {
        let h: isize = hands.next()?.parse().ok()?;
        let c: isize = currs.next()?.parse().ok()?;
        let m: usize = moves.next()?.parse().ok()?;
        res.push(((c, if d == "CW" { h } else { -h }), m));
    }

    Some(res)
}

fn normalize(hands: &mut [H]) -> isize {
    let mut n = 1;

    for ((_, step), _) in hands.iter() {
        n *= step.abs();
    }

    for ((pos, step), _) in hands.iter_mut() {
        *step = n / *step;
        *pos *= (*step).abs();
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let sample = "\
hands 6:8:12
direction CW:CCW:CW
current 2:4:8
move 9:7:4";

        let mut out = Vec::new();
        solve1(&mut sample.lines(), &mut out);

        assert_eq!(out, b"5\n");
    }

    #[test]
    fn test2() {
        let sample = "\
hands 6:8:12
direction CW:CCW:CW
current 1:2:2
move 10:12:15";

        let mut out = Vec::new();
        solve1(&mut sample.lines(), &mut out);

        assert_eq!(out, b"7\n");
    }

    #[test]
    fn test3() {
        let sample = "\
hands 6:8:12
direction CW:CCW:CW
current 1:2:2
move 10:12:20";

        let mut out = Vec::new();
        solve1(&mut sample.lines(), &mut out);

        assert_eq!(out, b"9\n");
    }

    #[test]
    fn test4() {
        let sample = "\
hands 8:11:11
direction CCW:CW:CCW
current 6:6:8
move 70:50:40";

        let mut out = Vec::new();
        solve1(&mut sample.lines(), &mut out);

        assert_eq!(out, b"29\n");
    }

    #[test]
    fn test5() {
        let sample = "\
hands 7:7:26
direction CW:CCW:CCW
current 2:5:8
move 3113:2467:2492";

        let mut out = Vec::new();
        solve1(&mut sample.lines(), &mut out);

        assert_eq!(out, b"1597\n");
    }

    #[test]
    fn test6() {
        let sample = "\
hands 4:13:23
direction CCW:CCW:CCW
current 0:5:21
move 3136:2208:3338";

        let mut out = Vec::new();
        solve1(&mut sample.lines(), &mut out);

        assert_eq!(out, b"1394\n");
    }

    #[test]
    fn test7() {
        let sample = "\
hands 9:5:26
direction CW:CCW:CW
current 7:2:25
move 3103:3153:2569";

        let mut out = Vec::new();
        solve1(&mut sample.lines(), &mut out);

        assert_eq!(out, b"1950\n");
    }

    #[test]
    fn test8() {
        let sample = "\
hands 5:7:9
direction CW:CCW:CW
current 0:0:0
move 2:8:11";

        let mut out = Vec::new();
        solve1(&mut sample.lines(), &mut out);

        assert_eq!(out, b"4\n");
    }
}
