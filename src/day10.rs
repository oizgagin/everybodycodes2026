use std::{
    collections::{BTreeMap, BTreeSet},
    hash::{DefaultHasher, Hash, Hasher},
    io::{self, Read, Write},
};

fn main() {
    let mut stdout = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf).unwrap();
    let stdin = buf.lines();

    //solve1(&mut stdin.clone(), &mut stdout);
    //solve2(&mut stdin.clone(), &mut stdout);
    solve3(&mut stdin.clone(), &mut stdout);
}

fn solve1<'a, W>(stdin: &mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let (cages, mut ghosts) = parse(stdin, 10000);

    let mut res = 0;
    for _ in 1..=10 {
        let (collisions, next_ghosts) = move_ghosts(&ghosts, &cages);
        res += collisions;
        ghosts = next_ghosts;
    }

    writeln!(stdout, "{res}").unwrap();
}

fn solve2<'a, W>(stdin: &mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let (cages, mut ghosts) = parse(stdin, 10000);

    let mut rounds = 0;
    while !ghosts.is_empty() {
        let (_, next_ghosts) = move_ghosts(&ghosts, &cages);
        ghosts = next_ghosts;
        rounds += 1;
    }

    writeln!(stdout, "{rounds}").unwrap();
}

fn solve3<'a, W>(stdin: &mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let (cages, mut ghosts) = parse(stdin, 1_000_000_000_000);

    let mut seen: BTreeMap<u64, (usize, BTreeMap<usize, G>)> = BTreeMap::new();

    let mut round = 0;

    while !ghosts.is_empty() {
        let (_, mut next_ghosts) = move_ghosts(&ghosts, &cages);
        round += 1;

        let h = hash_ghosts(&next_ghosts);
        if let Some((prev_round, prev_ghosts)) = seen.get(&h) {
            let cycle_len = round - prev_round;

            let weakest = next_ghosts.values().map(|&g| g.2).min().unwrap();
            let cycles = weakest / cycle_len;

            for (ghost_id, &(_, _, prev_collisions)) in prev_ghosts {
                assert!(next_ghosts.contains_key(ghost_id));

                let collisions_per_cycle = prev_collisions - next_ghosts.get(ghost_id).unwrap().2;

                next_ghosts
                    .entry(*ghost_id)
                    .and_modify(|g| g.2 = g.2.saturating_sub(cycles * collisions_per_cycle));
            }

            next_ghosts.retain(|_, &mut (_, _, left)| left > 0);

            round += cycles * cycle_len;
        }

        seen.insert(h, (round, next_ghosts.clone()));
        ghosts = next_ghosts;
    }

    writeln!(stdout, "{round}").unwrap();
}

fn parse<'a>(lines: &mut impl Iterator<Item = &'a str>, left_collisions: usize) -> M {
    let mut ghost_id = 0;

    let mut cages = BTreeSet::new();
    let mut ghosts = BTreeMap::new();

    for (i, line) in lines.enumerate() {
        for (j, b) in line.bytes().enumerate() {
            match b {
                b'#' => {
                    cages.insert((i, j));
                }
                g @ (b'<' | b'>' | b'^' | b'v') => {
                    ghost_id += 1;
                    ghosts.insert(ghost_id, ((i, j), dir(g), left_collisions));
                }
                _ => {}
            }
        }
    }

    (cages, ghosts)
}

fn move_ghosts(ghosts: &BTreeMap<usize, G>, cages: &BTreeSet<P>) -> (usize, BTreeMap<usize, G>) {
    let mut collisions = 0;
    let mut next_ghosts: BTreeMap<usize, G> = BTreeMap::new();

    // move ghosts, rotate if cage is hit
    for (&ghost_id, &(p, d, left)) in ghosts {
        let next_p = add_p(p, d);

        if cages.contains(&next_p) {
            collisions += 1;

            if left == 1 {
                continue;
            }

            next_ghosts.insert(ghost_id, (p, clockwise(d), left - 1));
        } else {
            next_ghosts.insert(ghost_id, (next_p, d, left));
        }
    }

    // check ghosts collisions
    for ghost_ids in group_ghosts(&next_ghosts).values() {
        if ghost_ids.len() > 1 {
            collisions += ghost_ids.len();

            for &(ghost_id, _) in ghost_ids {
                let ghost = next_ghosts.get_mut(&ghost_id).unwrap();
                ghost.1 = turnaround(ghost.1);
                ghost.2 -= 1;
            }
        }
    }

    next_ghosts.retain(|_, &mut (_, _, left)| left > 0);

    (collisions, next_ghosts)
}

fn group_ghosts(ghosts: &BTreeMap<usize, G>) -> BTreeMap<P, Vec<(usize, D)>> {
    let mut by_p: BTreeMap<P, Vec<(usize, D)>> = BTreeMap::new();
    for (&ghost_id, &(p, d, _)) in ghosts {
        by_p.entry(p).or_default().push((ghost_id, d));
    }
    by_p
}

fn hash_ghosts(ghosts: &BTreeMap<usize, G>) -> u64 {
    let mut hasher = DefaultHasher::new();
    group_ghosts(ghosts).hash(&mut hasher);
    hasher.finish()
}

type P = (usize, usize);
type D = (isize, isize);

// cage cells, ghosts: id -> (curr coord, curr dir, left collisions)
type G = (P, D, usize);
type M = (BTreeSet<P>, BTreeMap<usize, G>);

fn dir(b: u8) -> D {
    match b {
        b'^' => (-1, 0),
        b'v' => (1, 0),
        b'<' => (0, -1),
        b'>' => (0, 1),
        _ => unimplemented!(),
    }
}

fn clockwise(d: D) -> D {
    match d {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => unimplemented!(),
    }
}

fn turnaround(d: D) -> D {
    match d {
        (-1, 0) => (1, 0),
        (1, 0) => (-1, 0),
        (0, 1) => (0, -1),
        (0, -1) => (0, 1),
        _ => unimplemented!(),
    }
}

fn add_p(p: P, d: D) -> P {
    let (i, j) = p;
    let (di, dj) = d;
    (
        i.checked_add_signed(di).unwrap(),
        j.checked_add_signed(dj).unwrap(),
    )
}
