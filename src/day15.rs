use std::{
    collections::{HashSet, VecDeque},
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
    let (grid, n, m, deposits, start, end) = parse(stdin);
    let target_mask = (1 << deposits.len()) - 1;

    // curr point, point from, collected deposits bitmask, dist
    type SD = (P, P, usize, usize);
    type S = (P, P, usize);

    let mut q: VecDeque<SD> = VecDeque::new();
    let mut inq: HashSet<S> = HashSet::new();

    q.push_back((start, start, 0, 0));
    inq.insert((start, start, 0));

    while !q.is_empty() {
        let (currp, fromp, mask, dist) = q.pop_front().unwrap();

        if currp == end && mask == target_mask {
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

                match grid[nexti][nextj] {
                    b'@' => {
                        let i = deposits.iter().position(|&p| p == nextp).unwrap();
                        if inq.insert((nextp, currp, mask | (1 << i))) {
                            q.push_back((nextp, currp, mask | (1 << i), dist + 1));
                        }
                    }
                    b'.' | b'E' if inq.insert((nextp, currp, mask)) => {
                        q.push_back((nextp, currp, mask, dist + 1));
                    }
                    _ => {}
                }
            }
        }
    }
}

fn solve2<W>(stdin: &str, stdout: &mut W)
where
    W: Write,
{
    let (grid, n, m, _, start, end) = parse(stdin);

    // curr point, point from, collected, dist
    type SD = (P, P, usize, usize);
    type S = (P, P, usize);

    let mut q: VecDeque<SD> = VecDeque::new();
    let mut inq: HashSet<S> = HashSet::new();

    q.push_back((start, start, 0, 0));
    inq.insert((start, start, 0));

    while !q.is_empty() {
        let (currp, fromp, collected, dist) = q.pop_front().unwrap();

        if currp == end && collected >= 2026 {
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

                match grid[nexti][nextj] {
                    b'@' => {
                        if inq.insert((nextp, currp, collected + 1)) {
                            q.push_back((nextp, currp, collected + 1, dist + 1));
                        }
                    }
                    b'.' | b'E' if inq.insert((nextp, currp, collected)) => {
                        q.push_back((nextp, currp, collected, dist + 1));
                    }
                    _ => {}
                }
            }
        }
    }
}

const DIRS: &[(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

type P = (usize, usize);

// grid, n, m, deposits, start, end
type G = (Vec<Vec<u8>>, usize, usize, Vec<P>, P, P);

fn parse(s: &str) -> G {
    let grid: Vec<Vec<u8>> = s
        .split_ascii_whitespace()
        .map(|l| l.as_bytes().to_vec())
        .collect();

    let (n, m) = (grid.len(), grid[0].len());

    let mut deposits = Vec::new();
    let (mut start, mut end) = ((0, 0), (0, 0));

    for i in 0..n {
        for j in 0..m {
            match grid[i][j] {
                b'@' => {
                    deposits.push((i, j));
                }
                b'S' => {
                    start = (i, j);
                }
                b'E' => {
                    end = (i, j);
                }
                _ => {}
            }
        }
    }

    (grid, n, m, deposits, start, end)
}
