use std::{
    io::{self, Read, Write},
    mem::swap,
};

fn main() {
    let mut stdout = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf).unwrap();

    let map = parse(buf);
    solve1(&mut stdout, &map);
    solve2(&mut stdout, &map);
    solve3(&mut stdout, &map);
}

fn parse(input: String) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.as_bytes().iter().map(|&b| b - b'0').collect())
        .collect()
}

fn solve1<W>(w: &mut W, map: &[Vec<u8>])
where
    W: Write,
{
    let (n, m) = (map.len(), map[0].len());

    let mut curr = vec![vec![b'.'; m]; n];
    let mut next = vec![vec![b'.'; m]; n];

    curr[n / 2][m / 2] = b'#';

    while eval_land(&curr, &mut next, map) {
        swap(&mut curr, &mut next);
    }

    let res = curr.into_iter().flatten().filter(|&b| b == b'#').count();
    writeln!(w, "{res}").unwrap();
}

fn solve2<W>(w: &mut W, map: &[Vec<u8>])
where
    W: Write,
{
    let (n, m) = (map.len(), map[0].len());

    let mut curr = vec![vec![b'.'; m]; n];
    let mut next = vec![vec![b'.'; m]; n];

    curr[n / 2][m / 2] = b'#';

    while eval_land(&curr, &mut next, map) {
        swap(&mut curr, &mut next);
    }

    while eval_sand(&curr, &mut next) {
        swap(&mut curr, &mut next);
    }

    let res = curr.into_iter().flatten().filter(|&b| b == b'=').count();
    writeln!(w, "{res}").unwrap();
}

fn solve3<W>(w: &mut W, map: &[Vec<u8>])
where
    W: Write,
{
    let (n, m) = (map.len(), map[0].len());

    let mut curr = vec![vec![b'.'; m]; n];
    let mut next = vec![vec![b'.'; m]; n];

    curr[n / 2][m / 2] = b'#';

    while eval_land(&curr, &mut next, map) {
        swap(&mut curr, &mut next);
    }

    while eval_sand(&curr, &mut next) {
        swap(&mut curr, &mut next);
    }

    // place trees
    for i in 0..n {
        for j in 0..m {
            if curr[i][j] != b'#' {
                continue;
            }

            let mut ok = true;
            for (ni, nj) in neighs(i, j, n, m, 2) {
                if ni.abs_diff(i) == 2 && nj.abs_diff(j) == 2 {
                    continue;
                }

                if curr[ni][nj] == b'.' || curr[ni][nj] == b'=' {
                    ok = false;
                    break;
                }
            }

            if ok {
                curr[i][j] = b'@';
            }
        }
    }

    // grow forest
    (0..2).for_each(|_| {
        grow_forest(&curr, &mut next);
        swap(&mut curr, &mut next);
    });

    let res = curr.into_iter().flatten().filter(|&b| b == b'@').count();
    writeln!(w, "{res}").unwrap();
}

fn neighs(
    i: usize,
    j: usize,
    n: usize,
    m: usize,
    d: isize,
) -> impl Iterator<Item = (usize, usize)> {
    (-d..=d).flat_map(move |di| {
        (-d..=d).filter_map(move |dj| {
            let ni = i.checked_add_signed(di)?;
            let nj = j.checked_add_signed(dj)?;
            ((di != 0 || dj != 0) && ni < n && nj < m).then_some((ni, nj))
        })
    })
}

fn eval_land(curr: &[Vec<u8>], next: &mut [Vec<u8>], map: &[Vec<u8>]) -> bool {
    let (n, m) = (curr.len(), curr[0].len());

    let mut changed = false;

    for i in 0..n {
        for j in 0..m {
            if curr[i][j] == b'#' {
                next[i][j] = b'#';
                continue;
            }

            let mut darks = 0;
            let mut lands = 0;
            for (ni, nj) in neighs(i, j, n, m, 1) {
                if curr[ni][nj] == b'#' {
                    lands += 1;
                    continue;
                }
                if map[ni][nj] == 1 {
                    darks += 1;
                }
            }

            if lands > 0 && (lands + darks >= 5) {
                next[i][j] = b'#';
                changed = true;
            }
        }
    }

    changed
}

fn eval_sand(curr: &[Vec<u8>], next: &mut [Vec<u8>]) -> bool {
    let (n, m) = (curr.len(), curr[0].len());

    let mut changed = false;

    for i in 0..n {
        for j in 0..m {
            if curr[i][j] == b'.' {
                continue;
            }

            if curr[i][j] == b'=' {
                next[i][j] = b'=';
                continue;
            }

            if (i == 0 || j == 0 || i == n - 1 || j == m - 1) && next[i][j] != b'=' {
                changed = true;
                next[i][j] = b'=';
                continue;
            }

            let mut sand = 0;
            for (ni, nj) in neighs(i, j, n, m, 1) {
                if curr[ni][nj] == b'.' {
                    next[i][j] = b'=';
                    changed = true;
                    break;
                }

                if curr[ni][nj] == b'=' {
                    sand += 1;
                }
            }

            if sand >= 5 {
                next[i][j] = b'=';
                changed = true;
            }
        }
    }

    changed
}

fn grow_forest(curr: &[Vec<u8>], next: &mut [Vec<u8>]) {
    let (n, m) = (curr.len(), curr[0].len());

    for i in 0..n {
        for j in 0..m {
            next[i][j] = curr[i][j];

            if curr[i][j] != b'#' {
                continue;
            }

            let mut trees = 0;
            let mut sands = 0;

            for (ni, nj) in neighs(i, j, n, m, 1) {
                if curr[ni][nj] == b'=' {
                    sands += 1;
                    break;
                }
                if curr[ni][nj] == b'@' {
                    trees += 1;
                }
            }

            if sands == 0 && trees >= 4 {
                next[i][j] = b'@';
            }
        }
    }
}
