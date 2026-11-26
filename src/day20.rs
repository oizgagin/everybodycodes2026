use std::io::{self, Read, Write};

fn main() {
    let mut stdout = io::BufWriter::new(io::stdout().lock());

    let mut stdin = String::new();
    io::stdin().lock().read_to_string(&mut stdin).unwrap();

    solve1(&stdin, &mut stdout);
}

fn solve1<W>(stdin: &str, stdout: &mut W)
where
    W: Write,
{
    let exprs = parse(stdin);

    let mut res = 0;
    for (e1, e2) in exprs {
        if eval(e1) == eval(e2) {
            res += 2 + e_segs(e1) + e_segs(e2);
        }
    }

    let res = to_base11(res);

    writeln!(stdout, "{res}").unwrap();
}

// x, y, op
type E<'a> = (&'a str, &'a str, u8);

fn parse(s: &str) -> Vec<(E<'_>, E<'_>)> {
    s.lines()
        .filter_map(|l| l.split_once('='))
        .map(|(e1, e2)| {
            let e1: Vec<&str> = e1.split_ascii_whitespace().collect();
            let e2: Vec<&str> = e2.split_ascii_whitespace().collect();

            (
                (e1[0], e1[2], e1[1].as_bytes()[0]),
                (e2[0], e2[2], e2[1].as_bytes()[0]),
            )
        })
        .collect()
}

fn eval(e: E) -> usize {
    let (x, y, op) = e;

    match op {
        b'+' => from_base11(x) + from_base11(y),
        b'-' => from_base11(x) - from_base11(y),
        _ => unimplemented!(),
    }
}

fn e_segs(e: E) -> usize {
    s_segs(e.0) + s_segs(e.1) + b_segs(e.2)
}

fn s_segs(s: &str) -> usize {
    s.bytes().map(b_segs).sum()
}

fn b_segs(b: u8) -> usize {
    match b {
        b'-' => 1,
        b'1' | b'+' => 2,
        b'7' => 3,
        b'4' => 4,
        b'2' | b'3' | b'5' => 5,
        b'6' | b'0' | b'9' | b'A' => 6,
        b'8' => 7,
        _ => unimplemented!(),
    }
}

fn from_base11(s: &str) -> usize {
    let mut res = 0;
    let mut p = 1;
    for d in s.bytes().rev() {
        res += p * from_base11_d(d);
        p *= 11;
    }
    res
}

fn from_base11_d(d: u8) -> usize {
    match d {
        b @ (b'0'..=b'9') => (b - b'0') as usize,
        b'A' => 10,
        _ => unimplemented!(),
    }
}

fn to_base11(mut v: usize) -> String {
    let mut bytes = Vec::new();

    loop {
        let d = v % 11;
        bytes.push(to_base11_d(d));
        v /= 11;

        if v == 0 {
            break;
        }
    }

    bytes.reverse();

    String::from_utf8(bytes).unwrap()
}

fn to_base11_d(d: usize) -> u8 {
    match d {
        0..=9 => b'0' + (d as u8),
        10 => b'A',
        _ => unimplemented!(),
    }
}
