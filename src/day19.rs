use std::{
    cmp::{max, min},
    collections::BTreeMap,
    io::{self, Read, Write},
};

fn main() {
    let mut stdout = io::BufWriter::new(io::stdout().lock());

    let mut stdin = String::new();
    io::stdin().lock().read_to_string(&mut stdin).unwrap();

    //solve1(&stdin, &mut stdout);
    solve2(&stdin, &mut stdout);
}

fn solve1<W>(stdin: &str, stdout: &mut W)
where
    W: Write,
{
    let (g1, g2) = parse(stdin);

    let outs = g1
        .iter()
        .zip(g2.iter())
        .flat_map(|(r1, r2)| r1.iter().zip(r2.iter()))
        .map(|(&c1, &c2)| combine(c1, c2));

    let mut c = BTreeMap::new();
    for out in outs {
        *c.entry(out).or_insert(0) += 1;
    }

    let mut freqs: Vec<usize> = c.values().copied().collect();
    freqs.sort_unstable();

    let res: usize = freqs.iter().rev().take(3).product();
    writeln!(stdout, "{res}").unwrap();
}

fn solve2<W>(stdin: &str, stdout: &mut W)
where
    W: Write,
{
    let (g1, g2) = parse(stdin);

    let mut perm = vec![b'C', b'K', b'M', b'Y'];

    let mut res = 0;

    loop {
        let mut g3 = g2.clone();
        let (n, m) = (g3.len(), g3[0].len());

        for i in 0..n {
            for j in 0..m {
                g3[i][j] = perm[(g3[i][j] - b'1') as usize];
            }
        }

        let outs = g1
            .iter()
            .zip(g3.iter())
            .flat_map(|(r1, r2)| r1.iter().zip(r2.iter()))
            .map(|(&c1, &c2)| combine(c1, c2));

        let mut c = BTreeMap::new();
        for out in outs {
            *c.entry(out).or_insert(0) += 1;
        }

        let mut freqs: Vec<usize> = c.values().copied().collect();
        freqs.sort_unstable();

        let v: usize = freqs.iter().rev().take(3).product();

        res = max(res, v);

        if !next_permutation(&mut perm) {
            break;
        }
    }

    writeln!(stdout, "{res}").unwrap();
}

fn combine(c1: u8, c2: u8) -> u8 {
    let (c1, c2) = (min(c1, c2), max(c1, c2));

    match (c1, c2) {
        (_, b'K') | (b'K', _) => b'K',
        (b'C', b'M') => b'B',
        (b'C', b'Y') => b'G',
        (b'M', b'Y') => b'R',
        (_, _) if c1 == c2 => c1,
        _ => unimplemented!(),
    }
}

type G = Vec<Vec<u8>>;

fn parse(s: &str) -> (G, G) {
    let (g1, g2) = s.split_once("\n\n").unwrap();

    (
        g1.lines().map(|r| r.as_bytes().to_vec()).collect(),
        g2.lines().map(|r| r.as_bytes().to_vec()).collect(),
    )
}

fn next_permutation(v: &mut [u8]) -> bool {
    let n = v.len();

    for i in (1..n).rev() {
        if v[i - 1] < v[i] {
            let mut minj = n - 1;
            for j in (i..n).rev() {
                if v[j] > v[i - 1] {
                    minj = j;
                    break;
                }
            }

            v.swap(i - 1, minj);
            v[i..n].reverse();
            return true;
        }
    }

    false
}
