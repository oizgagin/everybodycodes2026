use std::{
    collections::BTreeMap,
    io::{self, Read, Write},
    mem::swap,
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
    let trees = parse(stdin);
    let res: usize = trees.iter().map(|&tree| grow(tree)).sum();
    writeln!(stdout, "{res}").unwrap();
}

fn solve2<'a, W>(stdin: &mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let trees = parse(stdin);
    let res: usize = trees.iter().map(|&tree| grow2(tree)).sum();
    writeln!(stdout, "{res}").unwrap();
}

fn solve3<'a, W>(stdin: &mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let trees = parse(stdin);
    let res: usize = trees.iter().map(|&tree| grow2(tree)).sum();
    writeln!(stdout, "{res}").unwrap();
}

fn grow(tree: (usize, usize, usize)) -> usize {
    let (_, growth, limit) = tree;

    let mut curr = vec![0; growth + 1];
    curr[1] = 1;

    let mut p = 0;

    let mut next = vec![0; growth + 1];

    let mut seconds = 1;

    let elements = |p: usize, curr: &Vec<usize>| {
        p + curr.iter().enumerate().map(|(i, &c)| i * c).sum::<usize>()
    };

    while elements(p, &curr) < limit {
        next.fill(0);

        next[2..=growth].copy_from_slice(&curr[1..growth]);

        p += next[growth];
        next[1] += next[growth];
        next[growth / 2] += 2 * next[growth];
        next[growth] = 0;

        swap(&mut curr, &mut next);

        seconds += 1;
    }

    seconds
}

fn grow2(tree: (usize, usize, usize)) -> usize {
    let (_, growth, limit) = tree;

    let mut curr: BTreeMap<usize, usize> = BTreeMap::new();
    curr.insert(1, 1);

    let mut p = 0;

    let mut next = BTreeMap::new();

    let mut seconds = 1;

    while p + 1 < limit {
        next.clear();

        let m = curr.keys().next_back().unwrap();

        // advance time until m = growth
        let d = growth - m;
        seconds += d;

        // update all branches
        for (&b, &c) in &curr {
            *next.entry(b + d).or_insert(0) += c;
        }

        // split branches
        let to_split = *next.get(&growth).unwrap();

        p += to_split;
        *next.entry(1).or_insert(0) += to_split;
        *next.entry(growth / 2).or_insert(0) += 2 * to_split;
        next.remove(&growth).unwrap();

        swap(&mut curr, &mut next);
    }

    seconds
}

fn parse<'a>(stdin: &mut impl Iterator<Item = &'a str>) -> Vec<(usize, usize, usize)> {
    stdin
        .filter_map(|l| {
            let mut it = l.split(", ");
            Some((
                it.next()?.strip_prefix("id=")?.parse().ok()?,
                it.next()?.strip_prefix("growth=")?.parse().ok()?,
                it.next()?.strip_prefix("limit=")?.parse().ok()?,
            ))
        })
        .collect()
}
