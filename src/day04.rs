use std::{
    cmp::Reverse,
    collections::BTreeMap,
    io::{self, Read, Write},
};

fn main() {
    let mut stdout = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf).unwrap();

    let results = parse(buf);
    solve1(&mut stdout, &results);
    solve2(&mut stdout, &results);
    solve3(&mut stdout, &results);
}

fn parse(input: String) -> Vec<(u64, u64, u64, u64)> {
    input
        .lines()
        .map(|line| {
            let mut it = line
                .split_ascii_whitespace()
                .map(|p| p.split('=').next_back().unwrap().parse::<u64>().unwrap());

            (
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
            )
        })
        .collect()
}

fn solve1<W>(w: &mut W, results: &Vec<(u64, u64, u64, u64)>)
where
    W: Write,
{
    let mut times = BTreeMap::new();
    for (id, _, _, time) in results {
        *times.entry(id).or_insert(0) += time;
    }

    let mut values: Vec<u64> = times.into_values().collect();
    values.sort_unstable();

    let res: u64 = values
        .iter()
        .enumerate()
        .map(|(pos, total_time)| ((pos + 1) as u64) * total_time)
        .sum();

    writeln!(w, "{res}").unwrap();
}

fn solve2<W>(w: &mut W, results: &Vec<(u64, u64, u64, u64)>)
where
    W: Write,
{
    let mut times = BTreeMap::new();
    let mut parts = BTreeMap::new();
    for &(id, _, _, time) in results {
        *times.entry(id).or_insert(0) += time;
        *parts.entry(id).or_insert(0) += 1;
    }

    let mut ids: Vec<&u64> = parts.keys().collect();
    ids.sort_by_key(|&id| (Reverse(parts.get(id).unwrap()), times.get(id).unwrap()));

    let res: u64 = ids
        .iter()
        .enumerate()
        .map(|(pos, &id)| ((pos + 1) as u64) * times.get(id).unwrap())
        .sum();

    writeln!(w, "{res}").unwrap();
}

fn solve3<W>(w: &mut W, results: &Vec<(u64, u64, u64, u64)>)
where
    W: Write,
{
    let mut results: Vec<(usize, usize, usize, usize)> = results
        .iter()
        .map(|&(id, quest, part, time)| (id as usize, quest as usize, part as usize, time as usize))
        .collect();

    results.sort_by_key(|&t| (t.1, t.2));

    let Q = results.iter().map(|&t| t.1).max().unwrap();
    let ID = results.iter().map(|&t| t.0).max().unwrap();

    // quest -> part -> id -> combined time
    // we assume that each player completes parts always sequentially
    let mut C = vec![vec![vec![usize::MAX; ID + 1]; 3 + 1]; Q + 1];
    for &(id, quest, part, time) in &results {
        assert!(
            part != 0 && (part == 1 || C[quest][part - 1][id] != usize::MAX),
            "{id},{quest},{part},{time}"
        );
        C[quest][part][id] = if part == 1 { 0 } else { C[quest][part - 1][id] } + time;
    }

    // accepts ID+1 times and returns ID+1 points array
    #[allow(clippy::items_after_statements)]
    fn rankings(times: &[usize], mut score: usize) -> Vec<usize> {
        let mut indexed: Vec<(usize, &usize)> = times.iter().enumerate().collect();
        indexed.sort_unstable_by_key(|&(_, &time)| time);

        let mut res = vec![0; times.len()];

        for group in indexed.chunk_by(|&(_, &time1), &(_, &time2)| time1 == time2) {
            for &(id, &time) in group {
                if time == usize::MAX {
                    break;
                }
                res[id] = score;
            }
            score = score.saturating_sub(group.len());
        }

        res
    }

    // now for each (quest, part) we can sort
    // and calculate scores
    let TOP_SCORES: [usize; 4] = [0, 50, 100, 150];

    let mut points: Vec<usize> = vec![0; ID + 1];

    for q in 1..=Q {
        for p in 1..=3 {
            let r = rankings(&C[q][p], TOP_SCORES[p]);
            for id in 1..=ID {
                points[id] += r[id];
            }
        }
    }

    // and now we can do checksum
    points.sort_unstable();

    let points: Vec<usize> = points.into_iter().skip(1).rev().collect();

    let mut checksum = 0;

    let mut pos = 1;
    for group in points.chunk_by(|&p1, &p2| p1 == p2) {
        for &p in group {
            checksum += pos * p;
        }
        pos += group.len();
    }

    writeln!(w, "{checksum}").unwrap();
}
