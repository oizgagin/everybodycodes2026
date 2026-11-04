use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, VecDeque},
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
    let (n, g, mut e) = build(stdin);

    let (start, _) = g
        .iter()
        .enumerate()
        .max_by_key(|&(_, conns)| conns.len())
        .unwrap();

    let mut curr = vec![0; n + 1];
    curr[start] = 1;

    for impulse in (1..=5).rev() {
        let mut next = vec![0; n + 1];

        for node in 1..=n {
            if curr[node] != 0 {
                e[node] += curr[node] * impulse;

                for &neigh in &g[node] {
                    next[neigh] += curr[node];
                }
            }
        }

        curr = next;
    }

    writeln!(stdout, "{}", e.iter().sum::<usize>()).unwrap();
}

fn solve2<'a, W>(stdin: &mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let (n, g, mut e) = build(stdin);

    let (start, _) = g
        .iter()
        .enumerate()
        .max_by_key(|&(_, conns)| conns.len())
        .unwrap();

    let mut curr = vec![0; n + 1];
    curr[start] = 1;

    for impulse in (1..=10).rev() {
        let mut next = vec![0; n + 1];

        for node in 1..=n {
            if curr[node] != 0 {
                e[node] += curr[node] * impulse;

                for &neigh in &g[node] {
                    next[neigh] += curr[node];
                }
            }
        }

        curr = next;
    }

    writeln!(stdout, "{}", e.iter().sum::<usize>()).unwrap();
}

fn solve3<'a, W>(stdin: &mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let (n, g, e) = build(stdin);

    #[allow(clippy::items_after_statements)]
    fn run(n: usize, g: &G, e: &mut [usize], start: usize, max_energy: usize) -> usize {
        // total incoming impulses to the node per impulse power
        let mut ins = vec![vec![0; 11]; n + 1];
        ins[start][10] = 1;

        // exploded nodes
        let mut exploded = vec![false; n + 1];

        loop {
            let mut next_ins = vec![vec![0; 11]; n + 1];

            // receive impulses
            for node in 1..=n {
                for impulse in 1..=10 {
                    if ins[node][impulse] > 0 {
                        assert!(!exploded[node]);

                        e[node] += ins[node][impulse] * impulse;

                        // pass impulses
                        if impulse > 1 {
                            for &neigh in &g[node] {
                                if !exploded[neigh] {
                                    next_ins[neigh][impulse - 1] += ins[node][impulse];
                                }
                            }
                        }
                    }
                }
            }

            // explode
            for node in 1..=n {
                if e[node] >= max_energy {
                    exploded[node] = true;
                    e[node] = 0;

                    // pass explode impulses
                    for &neigh in &g[node] {
                        if !exploded[neigh] {
                            next_ins[neigh][10] += 1;
                        }
                    }

                    // remove exploded from next receivers
                    next_ins[node].fill(0);
                }
            }

            // check if we have nothing else to pass
            let mut done = true;
            for node in 1..=n {
                if next_ins[node].iter().any(|&v| v > 0) {
                    assert!(!exploded[node]);
                    done = false;
                }
            }

            if done {
                break;
            }

            ins = next_ins;
        }

        e.iter().sum()
    }

    let mut res = 0;
    for start in 1..=n {
        let v = run(n, &g, &mut e.clone(), start, 1000000);
        res = max(res, v);
    }

    writeln!(stdout, "{res}").unwrap();
}

type G = Vec<Vec<usize>>;

fn build<'a>(stdin: &mut impl Iterator<Item = &'a str>) -> (usize, G, Vec<usize>) {
    type Cloud = (usize, usize, Vec<usize>);

    fn parse<'a>(it: &mut impl Iterator<Item = &'a str>) -> Option<Cloud> {
        let id = it.next()?.strip_circumfix("id=", ",")?.parse().ok()?;
        let energy = it.next()?.strip_circumfix("energy=", ",")?.parse().ok()?;
        let conns = it
            .next()?
            .strip_prefix("connections=")?
            .split(',')
            .flat_map(|s| s.parse().ok())
            .collect();

        Some((id, energy, conns))
    }

    let mut rows = Vec::new();
    while let Some(cloud) = parse(stdin) {
        rows.push(cloud);
    }

    let n = rows.len();

    let mut g = vec![Vec::new(); n + 1];
    let mut e = vec![0; n + 1];

    for (id, energy, conns) in rows {
        g[id] = conns;
        e[id] = energy;
    }

    (n, g, e)
}
