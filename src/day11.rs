use std::{
    cmp::min,
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
    let (n, g1, g2) = parse(stdin, 1).unwrap();

    let strain1 = strain(n, &floyd_warshall(n, &g1));
    let strain2 = strain(n, &floyd_warshall(n, &g2));

    let res = strain1 - strain2;
    writeln!(stdout, "{res}").unwrap();
}

fn solve2<'a, W>(stdin: &mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let (n, g1, g2) = parse(stdin, 3).unwrap();

    let strain1 = strain(n, &floyd_warshall(n, &g1));
    let mut strain2 = strain(n, &floyd_warshall(n, &g2));

    for (u, v) in coords(n) {
        if g2[u][v] == usize::MAX {
            let mut g3 = g2.clone();
            add_edge(&mut g3, u, v, 1);

            strain2 = min(strain2, strain(n, &floyd_warshall(n, &g3)));
        }
    }

    let res = strain1 - strain2;
    writeln!(stdout, "{res}").unwrap();
}

fn solve3<'a, W>(stdin: &mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let (n, g1, g2) = parse(stdin, 3).unwrap();

    let strain1 = strain(n, &floyd_warshall(n, &g1));
    let mut strain2 = strain(n, &floyd_warshall(n, &g2));

    let d = n / 8;
    for pivot in 1..=d {
        // crystals are located at (pivot, pivot + d, pivot + 2d, ...)
        // iterate over all possible pairs of crystals, add new edges
        // do floyd warshall
        let crystals: Vec<usize> = (pivot..=n).step_by(d).collect();
        assert_eq!(crystals.len(), 8);

        for pairs in all_pairs(crystals.len()) {
            assert_eq!(pairs.len(), 4);

            let mut g3 = g2.clone();

            for (i, j) in pairs {
                let (u, v) = (crystals[i], crystals[j]);
                if g3[u][v] == usize::MAX {
                    add_edge(&mut g3, u, v, 1);
                }
            }

            strain2 = min(strain2, strain(n, &floyd_warshall(n, &g3)));
        }
    }

    let res = strain1 - strain2;
    writeln!(stdout, "{res}").unwrap();
}

type G = Vec<Vec<usize>>;

fn parse<'a>(stdin: &mut impl Iterator<Item = &'a str>, k: usize) -> Option<(usize, G, G)> {
    stdin.next()?;

    let n = stdin.next()?.parse().ok()?;

    let additional: Vec<(usize, usize)> = std::iter::from_fn(|| {
        let u = stdin.next()?.parse().ok()?;
        let v = stdin.next()?.parse().ok()?;
        Some((u, v))
    })
    .collect();

    let mut g1 = vec![vec![usize::MAX; n + 1]; n + 1];

    for u in 1..=n {
        add_edge(&mut g1, u, u, 0);

        let mut pu = u;
        let mut nu = u;

        for _ in 1..=k {
            pu = prev(n, pu);
            nu = next(n, nu);

            add_edge(&mut g1, u, pu, 1);
            add_edge(&mut g1, u, nu, 1);
        }
    }

    let mut g2 = g1.clone();
    for (u, v) in additional {
        add_edge(&mut g2, u, v, 1);
    }

    Some((n, g1, g2))
}

fn add_edge(g: &mut G, u: usize, v: usize, d: usize) {
    assert!((u == v && d == 0) || u != v, "u = {}, v = {}", u, v);
    g[u][v] = d;
    g[v][u] = d;
}

fn floyd_warshall(n: usize, g: &G) -> G {
    let mut dists = g.clone();

    for k in 1..=n {
        for (i, j) in coords(n) {
            if dists[i][j] > dists[i][k].saturating_add(dists[k][j]) {
                dists[i][j] = dists[i][k] + dists[k][j];
            }
        }
    }

    dists
}

fn strain(n: usize, g: &G) -> usize {
    coords(n).map(|(u, v)| g[u][v]).sum()
}

fn coords(n: usize) -> impl Iterator<Item = (usize, usize)> {
    (1..=n).flat_map(move |u| (1..=n).map(move |v| (u, v)))
}

fn prev(n: usize, u: usize) -> usize {
    (u - 2 + n) % n + 1
}

fn next(n: usize, u: usize) -> usize {
    u % n + 1
}

#[allow(clippy::items_after_statements)]
fn all_pairs(n: usize) -> Vec<Vec<(usize, usize)>> {
    assert!(n % 2 == 0 && n <= 16);

    fn take(m: u16, b: usize) -> u16 {
        m & !(1 << b)
    }

    fn recurse(m: u16, curr: &mut Vec<(usize, usize)>, res: &mut Vec<Vec<(usize, usize)>>) {
        if m == 0 {
            res.push(curr.clone());
            return;
        }

        let v1 = m.trailing_zeros() as usize;

        let mut candidates = take(m, v1);

        while candidates != 0 {
            let v2 = candidates.trailing_zeros() as usize;

            curr.push((v1, v2));
            recurse(take(take(m, v1), v2), curr, res);
            curr.pop();

            candidates &= candidates - 1;
        }
    }

    let mut curr = Vec::new();
    let mut res = Vec::new();

    recurse((1u16 << n) - 1, &mut curr, &mut res);

    res
}
