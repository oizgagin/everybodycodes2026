use std::{
    cmp::{max, min},
    collections::BTreeSet,
    io::{self, Read, Write},
};

fn main() {
    let mut stdout = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf).unwrap();
    let stdin = buf.lines();

    solve1(&mut stdin.clone(), &mut stdout);
    solve2(&mut stdin.clone(), &mut stdout);
    solve3(&mut stdin.clone(), &mut stdout);
}

fn solve1<'a, W>(stdin: &mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let g = parse(stdin);
    let (n, m) = (g.len(), g[0].len());

    let mut blocks: Vec<Vec<(usize, usize)>> = Vec::new();

    let mut seen = BTreeSet::new();

    for i in 0..n {
        for j in 0..m {
            if g[i][j] == b'#' && !seen.contains(&(i, j)) {
                let mut block = BTreeSet::new();

                dfs(&g, i, j, n, m, &mut block);

                blocks.push(block.clone().into_iter().collect());
                seen.append(&mut block);
            }
        }
    }

    let mut res = 0;

    for block in &blocks {
        let (h, w) = hw(block);

        res += min(h, w);
    }

    writeln!(stdout, "{res}").unwrap();
}

fn solve2<'a, W>(stdin: &mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let g = parse(stdin);
    let (n, m) = (g.len(), g[0].len());

    let mut blocks: Vec<Vec<(usize, usize)>> = Vec::new();

    let mut seen = BTreeSet::new();

    for i in 0..n {
        for j in 0..m {
            if g[i][j] == b'#' && !seen.contains(&(i, j)) {
                let mut block = BTreeSet::new();

                dfs(&g, i, j, n, m, &mut block);

                blocks.push(block.clone().into_iter().collect());
                seen.append(&mut block);
            }
        }
    }

    let mut res = 0;

    for block in &blocks {
        let (h, w) = hw(block);

        if h * w == block.len() {
            res += min(h, w);
        } else if let Some(v) = l_shape(block) {
            res += v;
        }
    }

    writeln!(stdout, "{res}").unwrap();
}

fn solve3<'a, W>(stdin: &'a mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let g = parse(stdin);
    let (n, m) = (g.len(), g[0].len());

    let mut blocks: Vec<Vec<(usize, usize)>> = Vec::new();

    let mut seen = BTreeSet::new();

    for i in 0..n {
        for j in 0..m {
            if g[i][j] == b'#' && !seen.contains(&(i, j)) {
                let mut block = BTreeSet::new();

                dfs(&g, i, j, n, m, &mut block);

                blocks.push(block.clone().into_iter().collect());
                seen.append(&mut block);
            }
        }
    }

    let mut res = 0;

    for block in &blocks {
        let (h, w) = hw(block);

        if h * w == block.len() {
            res += min(h, w);
        } else if let Some(v) = l_shape(block) {
            res += v;
        } else {
            res += rect(block);
        }
    }

    writeln!(stdout, "{res}").unwrap();
}

type G = Vec<Vec<u8>>;

fn parse<'a>(stdin: &mut impl Iterator<Item = &'a str>) -> G {
    stdin.map(|l| l.as_bytes().to_owned()).collect()
}

const DIRS: &[(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

fn dfs(g: &G, i: usize, j: usize, n: usize, m: usize, seen: &mut BTreeSet<(usize, usize)>) {
    if !seen.insert((i, j)) {
        return;
    }

    for &(di, dj) in DIRS {
        let ni = i.checked_add_signed(di);
        let nj = j.checked_add_signed(dj);

        if let Some(ni) = ni
            && let Some(nj) = nj
            && ni < n
            && nj < m
            && g[ni][nj] == b'#'
        {
            dfs(g, ni, nj, n, m, seen);
        }
    }
}

fn hw(b: &[(usize, usize)]) -> (usize, usize) {
    let mini = b.iter().map(|c| c.0).min().unwrap();
    let maxi = b.iter().map(|c| c.0).max().unwrap();

    let minj = b.iter().map(|c| c.1).min().unwrap();
    let maxj = b.iter().map(|c| c.1).max().unwrap();

    (maxi - mini + 1, maxj - minj + 1)
}

fn l_shape(b: &[(usize, usize)]) -> Option<usize> {
    let mini = b.iter().map(|c| c.0).min().unwrap();
    let top = b.iter().filter(|(i, _)| *i == mini).count();

    let maxi = b.iter().map(|c| c.0).max().unwrap();
    let bottom = b.iter().filter(|(i, _)| *i == maxi).count();

    let minj = b.iter().map(|c| c.1).min().unwrap();
    let left = b.iter().filter(|(_, j)| *j == minj).count();

    let maxj = b.iter().map(|c| c.1).max().unwrap();
    let right = b.iter().filter(|(_, j)| *j == maxj).count();

    if left == right && top == bottom {
        return None;
    }

    let h = maxi - mini + 1;
    let w = maxj - minj + 1;

    assert_eq!(h, max(left, right));
    assert_eq!(w, max(top, bottom));

    let dv = min(left, right);
    let dh = min(top, bottom);

    Some(min(h, w).min(dv + dh))
}

fn rect(b: &[(usize, usize)]) -> usize {
    let mini = b.iter().map(|c| c.0).min().unwrap();
    let maxi = b.iter().map(|c| c.0).max().unwrap();
    let minj = b.iter().map(|c| c.1).min().unwrap();
    let maxj = b.iter().map(|c| c.1).max().unwrap();

    let full_rows: Vec<usize> = (mini..=maxi)
        .filter(|&i| (minj..=maxj).all(|j| b.contains(&(i, j))))
        .collect();

    let mut it = full_rows.chunk_by(|&r1, &r2| r1 + 1 == r2);
    let top = it.next().unwrap().len();
    let bottom = it.next().unwrap().len();

    let full_cols: Vec<usize> = (minj..=maxj)
        .filter(|&j| (mini..=maxi).all(|i| b.contains(&(i, j))))
        .collect();

    let mut it = full_cols.chunk_by(|&c1, &c2| c1 + 1 == c2);
    let left = it.next().unwrap().len();
    let right = it.next().unwrap().len();

    let h = maxi - mini + 1;
    let w = maxj - minj + 1;

    let mut res = left + right + top + bottom;

    res = min(res, left + h);
    res = min(res, right + h);

    res = min(res, bottom + w);
    res = min(res, top + w);

    res = min(res, top + bottom + 2 * (h - top - bottom));
    res = min(res, left + right + 2 * (w - left - right));

    res
}
