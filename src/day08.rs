use std::{
    cmp::max,
    io::{self, Read, Write},
};

fn main() {
    let mut stdout = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf).unwrap();
    let stdin = buf.split_ascii_whitespace();

    solve1(&mut stdin.clone(), &mut stdout);
    solve2(&mut stdin.clone(), &mut stdout);
    solve3(&mut stdin.clone(), &mut stdout);
}

fn solve1<'a, W>(stdin: &mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let g = parse(stdin);

    let (exterior, di, dj) = round(&g, 1);
    let res = pass(&g, exterior, di, dj).len();

    writeln!(stdout, "{res}").unwrap();
}

fn solve2<'a, W>(stdin: &mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let g = parse(stdin);

    let mut res = 0;
    for i in 1..=6 {
        let (exterior, di, dj) = round(&g, i);
        res += pass(&g, exterior, di, dj).len();
    }

    writeln!(stdout, "{res}").unwrap();
}

fn solve3<'a, W>(stdin: &mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let mut g = parse(stdin);

    let mut curr_round = 0;

    let mut warriors = 0;
    let mut total_energy = 0;

    while warriors < 100000 {
        let (exterior, di, dj) = round(&g, (curr_round % 6) + 1);

        let received_energy = pass(&g, exterior, di, dj);
        for (i, j) in received_energy {
            total_energy += 1;

            if let Some(v) = &mut g[i][j] {
                if *v == 9 {
                    warriors += 1;
                    *v = -3;
                } else {
                    *v += 1;
                }
            }
        }

        for row in &mut g {
            for col in row.iter_mut().flatten() {
                if *col <= 0 {
                    *col += 1;
                }
            }
        }

        curr_round += 1;
    }

    writeln!(stdout, "{total_energy}").unwrap();
}

type G = Vec<Vec<Option<i8>>>;

fn parse<'a>(stdin: &mut impl Iterator<Item = &'a str>) -> G {
    stdin
        .map(|line| {
            line.bytes()
                .map(|ch| {
                    if ch == b'.' {
                        None
                    } else {
                        Some((ch - b'0').cast_signed())
                    }
                })
                .collect()
        })
        .collect()
}

fn line(g: &G, i: usize, j: usize, di: isize, dj: isize) -> impl Iterator<Item = (usize, usize)> {
    let (n, m) = (g.len(), g[0].len());

    (0..).map_while(move |c| {
        let ni = i.checked_add_signed(c * di)?;
        let nj = j.checked_add_signed(c * dj)?;
        (ni < n && nj < m && g[ni][nj].is_some()).then_some((ni, nj))
    })
}

fn row(g: &G, i: usize) -> impl DoubleEndedIterator<Item = (usize, usize)> {
    g[i].iter()
        .enumerate()
        .filter_map(move |(j, &v)| v.map(|_| (i, j)))
}

fn first(g: &G, i: usize) -> (usize, usize) {
    row(g, i).next().unwrap()
}

fn last(g: &G, i: usize) -> (usize, usize) {
    row(g, i).last().unwrap()
}

// returns exterior + movement direction
fn round(g: &G, round: u32) -> (Box<dyn Iterator<Item = (usize, usize)> + '_>, isize, isize) {
    let n = g.len();

    match round {
        1 => {
            let N = row(g, 0);
            let NW = (1..=(n / 2)).map(move |i| first(g, i));
            (Box::new(N.chain(NW)), 1, 1)
        }
        2 => {
            let N = row(g, 0);
            let NE = (1..=(n / 2)).map(move |i| last(g, i));
            (Box::new(N.chain(NE)), 1, -1)
        }
        3 => {
            let NE = (0..(n / 2)).map(move |i| last(g, i));
            let SE = ((n / 2)..n).map(move |i| last(g, i));
            (Box::new(NE.chain(SE)), 0, -2)
        }
        4 => {
            let SE = ((n / 2)..n - 1).map(move |i| last(g, i));
            let S = row(g, n - 1);
            (Box::new(SE.chain(S)), -1, -1)
        }
        5 => {
            let S = row(g, n - 1);
            let SW = ((n / 2)..n - 1).map(move |i| first(g, i));
            (Box::new(S.chain(SW)), -1, 1)
        }

        6 => {
            let NW = (0..(n / 2)).map(move |i| first(g, i));
            let SW = ((n / 2)..n).map(move |i| first(g, i));
            (Box::new(NW.chain(SW)), 0, 2)
        }
        _ => unimplemented!(),
    }
}

fn pass(
    g: &G,
    exterior: impl Iterator<Item = (usize, usize)>,
    di: isize,
    dj: isize,
) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    for (i, j) in exterior {
        line(g, i, j, di, dj).fold(0, |shadow, (ni, nj)| {
            if shadow < g[ni][nj].unwrap() {
                res.push((ni, nj));
            }

            max(shadow, g[ni][nj].unwrap()).saturating_sub(1).max(0)
        });
    }

    res
}
