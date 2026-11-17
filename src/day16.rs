use std::{
    collections::{BTreeMap, HashSet, VecDeque},
    io::{self, Read, Write},
    mem::swap,
};

fn main() {
    let mut stdout = io::BufWriter::new(io::stdout().lock());

    let mut stdin = String::new();
    io::stdin().lock().read_to_string(&mut stdin).unwrap();

    solve1(&stdin, &mut stdout);
    solve2(&stdin, &mut stdout);
}

fn solve1<W>(stdin: &str, stdout: &mut W)
where
    W: Write,
{
    let mut s = stdin.trim().as_bytes().to_owned();
    let n = s.len();

    let (mut r, mut g, mut b, mut y) = (0, 0, 0, 0);
    for &v in &s {
        match v {
            b'R' => {
                r += 1;
            }
            b'G' => {
                g += 1;
            }
            b'B' => {
                b += 1;
            }
            b'Y' => {
                y += 1;
            }
            _ => unimplemented!(),
        }
    }

    let first = |s: &[u8], ch: u8| s.iter().position(|&v| v == ch).unwrap();
    let last = |s: &[u8], ch: u8| s.iter().rposition(|&v| v == ch).unwrap();
    let rev = |s: &mut [u8], l: usize, r: usize| s[l..=r].reverse();
    let count = |ch: u8| {
        if ch == b'R' {
            r
        } else if ch == b'G' {
            g
        } else if ch == b'B' {
            b
        } else {
            y
        }
    };
    let grouped = |s: &[u8], ch: u8| last(s, ch) - first(s, ch) + 1 == count(ch);
    let firstgroup =
        |s: &[u8], ch: u8| (first(s, ch)..).take_while(|&i| s[i] == ch).last().unwrap() + 1;

    let mut res = 0;
    loop {
        let mut done = true;
        for i in 0..n {
            if !grouped(&s, s[i]) {
                let (l, r) = (firstgroup(&s, s[i]), last(&s, s[i]));
                assert!(l < r, "{l}, {r}, {}", s[i] as char);
                rev(&mut s, l, r);
                res += 1;
                done = false;
            }
        }

        if done {
            break;
        }
    }

    writeln!(stdout, "{res}").unwrap();
}

fn solve2<W>(stdin: &str, stdout: &mut W)
where
    W: Write,
{
    let mut s = stdin.trim().repeat(10).as_bytes().to_owned();
    let n = s.len();

    let mut f = vec![0; 255];

    for &v in &s {
        f[v as usize] += 1;
    }

    let first = |s: &[u8], ch: u8| s.iter().position(|&v| v == ch).unwrap();
    let last = |s: &[u8], ch: u8| s.iter().rposition(|&v| v == ch).unwrap();
    let rev = |s: &mut [u8], l: usize, r: usize| s[l..=r].reverse();
    let count = |ch: u8| f[ch as usize];
    let grouped = |s: &[u8], ch: u8| last(s, ch) - first(s, ch) + 1 == count(ch);
    let firstgroup =
        |s: &[u8], ch: u8| (first(s, ch)..).take_while(|&i| s[i] == ch).last().unwrap() + 1;

    let mut res = 0;
    loop {
        let mut done = true;
        for i in 0..n {
            if !grouped(&s, s[i]) {
                let (l, r) = (firstgroup(&s, s[i]), last(&s, s[i]));
                assert!(l < r, "{l}, {r}, {}", s[i] as char);
                rev(&mut s, l, r);
                res += r - l + 1;
                done = false;
            }
        }

        if done {
            break;
        }
    }

    writeln!(stdout, "{res}").unwrap();
}
