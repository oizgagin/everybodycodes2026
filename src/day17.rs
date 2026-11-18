use std::{
    collections::{HashSet, VecDeque},
    io::{self, Read, Write},
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
    let (g, n, m) = parse(stdin);

    let mut q: VecDeque<(P, P, usize)> = VecDeque::new();
    let mut inq: HashSet<P> = HashSet::new();

    let target = (n - 1, m - 1);

    q.push_back(((0, 0), (0, 0), 0));
    inq.insert((0, 0));

    while !q.is_empty() {
        let (currp, fromp, dist) = q.pop_front().unwrap();

        if currp == target {
            writeln!(stdout, "{dist}").unwrap();
            break;
        }

        let (curri, currj) = currp;

        for &(di, dj) in DIRS {
            if let Some(nexti) = curri.checked_add_signed(di)
                && let Some(nextj) = currj.checked_add_signed(dj)
                && nexti < n
                && nextj < m
                && (nexti, nextj) != fromp
            {
                let nextp = (nexti, nextj);

                if g[nexti][nextj] == b'#' && inq.insert(nextp) {
                    q.push_back((nextp, currp, dist + 1));
                }
            }
        }
    }
}

fn solve2<W>(stdin: &str, stdout: &mut W)
where
    W: Write,
{
    let (g, n, m) = parse(stdin);

    let mut exterior = 0;
    for i in 0..n {
        for j in 0..m {
            if g[i][j] == b'.' {
                continue;
            }

            for &(di, dj) in DIRS {
                if let Some(ni) = i.checked_add_signed(di)
                    && let Some(nj) = j.checked_add_signed(dj)
                    && ni < n
                    && nj < m
                {
                    if g[ni][nj] == b'.' {
                        exterior += 1;
                    }
                } else {
                    exterior += 1;
                }
            }
        }
    }

    writeln!(stdout, "{exterior}").unwrap();
}

const DIRS: &[(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

type G = (Vec<Vec<u8>>, usize, usize);
type P = (usize, usize);

fn parse(s: &str) -> G {
    let grid: Vec<Vec<u8>> = s
        .split_ascii_whitespace()
        .map(|l| l.as_bytes().to_vec())
        .collect();

    let (n, m) = (grid.len(), grid[0].len());

    (grid, n, m)
}
