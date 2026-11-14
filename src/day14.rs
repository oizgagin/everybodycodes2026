use std::{
    cmp::min,
    collections::BTreeMap,
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
    let mut g = parse(stdin);
    let (n, m) = (g.len(), g[0].len());

    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            if g[i][j] != 0 {
                let v = g[i][j];
                res += dfs(&mut g, i, j, v);
            }
        }
    }

    writeln!(stdout, "{res}").unwrap();
}

fn solve2<'a, W>(stdin: &mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let g = parse(stdin);
    let res = kruskal(&dists(&g));
    writeln!(stdout, "{res}").unwrap();
}

fn solve3<'a, W>(stdin: &mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let g = parse(stdin);

    let d = dists(&g);
    let mut res = kruskal(&d);

    for i in 1..=255 {
        for j in i + 1..=255 {
            let mut dc = d.clone();

            for ni in 0..=255 {
                if dc[i][ni] != usize::MAX {
                    dc[i][ni] = 0;
                    dc[ni][i] = 0;
                }
                if dc[j][ni] != usize::MAX {
                    dc[j][ni] = 0;
                    dc[ni][j] = 0;
                }
            }

            res = min(res, kruskal(&dc));
        }
    }

    writeln!(stdout, "{res}").unwrap();
}

fn parse<'a>(stdin: &mut impl Iterator<Item = &'a str>) -> Vec<Vec<u8>> {
    stdin.map(|l| l.as_bytes().to_vec()).collect()
}

const DIRS: &[(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

fn dfs(g: &mut Vec<Vec<u8>>, i: usize, j: usize, v: u8) -> usize {
    let (n, m) = (g.len(), g[0].len());

    g[i][j] = 0;

    let mut res = 0;

    for &(di, dj) in DIRS {
        if let Some(ni) = i.checked_add_signed(di)
            && let Some(nj) = j.checked_add_signed(dj)
            && ni < n
            && nj < m
        {
            if g[ni][nj] == v {
                res += dfs(g, ni, nj, v);
            } else if g[ni][nj] != 0 {
                res += 1;
            }
        } else {
            res += 1;
        }
    }

    res
}

fn dists(g: &Vec<Vec<u8>>) -> Vec<Vec<usize>> {
    let (n, m) = (g.len(), g[0].len());

    let outer: usize = 0;
    let mut dists = vec![vec![usize::MAX; 256]; 256];

    for i in 0..n {
        for j in 0..m {
            let v = g[i][j] as usize;

            for &(di, dj) in DIRS {
                if let Some(ni) = i.checked_add_signed(di)
                    && let Some(nj) = j.checked_add_signed(dj)
                    && ni < n
                    && nj < m
                {
                    let nv = g[ni][nj] as usize;
                    if v != nv {
                        if dists[v][nv] == usize::MAX {
                            dists[v][nv] = 1;
                        } else {
                            dists[v][nv] += 1;
                        }
                    }
                } else if dists[outer][v] == usize::MAX {
                    dists[outer][v] = 1;
                    dists[v][outer] = 1;
                } else {
                    dists[outer][v] += 1;
                    dists[v][outer] += 1;
                }
            }
        }
    }

    dists
}

fn kruskal(dists: &Vec<Vec<usize>>) -> usize {
    let mut edges = Vec::new();
    for i in 0..=255 {
        for j in 0..=255 {
            if dists[i][j] != usize::MAX {
                edges.push((i, j, dists[i][j]));
            }
        }
    }

    edges.sort_unstable_by_key(|&(_, _, d)| d);

    let mut uf = uf::new();

    let mut res = 0;
    for (i, j, d) in edges {
        if uf::union(&mut uf, i, j) {
            res += d;
        }
    }

    res
}

mod uf {
    pub fn new() -> Vec<usize> {
        (0..=255).collect()
    }

    pub fn find(uf: &mut Vec<usize>, p: usize) -> usize {
        if p == uf[p] {
            return p;
        }

        uf[p] = find(uf, uf[p]);
        uf[p]
    }

    pub fn union(uf: &mut Vec<usize>, p1: usize, p2: usize) -> bool {
        let (pp1, pp2) = (find(uf, p1), find(uf, p2));
        if pp1 == pp2 {
            return false;
        }
        uf[pp1] = pp2;
        true
    }
}
