use std::{
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
    let (g, _) = build(stdin);
    writeln!(stdout, "{}", bfs(&g, "NULL").len() - 1).unwrap();
}

fn solve2<'a, W>(stdin: &mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let (_, revg) = build(stdin);
    writeln!(stdout, "{}", bfs(&revg, "NULL").len() - 1).unwrap();
}

fn solve3<'a, W>(stdin: &mut impl Iterator<Item = &'a str>, stdout: &mut W)
where
    W: Write,
{
    let (_, revg) = build(stdin);

    let mut c = BTreeMap::new();
    for &null in revg.keys().filter(|k| k.starts_with("NULL")) {
        for &node in &bfs(&revg, null) {
            *c.entry(node).or_insert(0) += 1;
        }
    }

    let res = c.values().filter(|&v| *v >= 2).count();
    writeln!(stdout, "{res}").unwrap();
}

type G<'a> = BTreeMap<&'a str, Vec<&'a str>>;

fn build<'a>(stdin: &mut impl Iterator<Item = &'a str>) -> (G<'a>, G<'a>) {
    let mut g = G::new();
    let mut revg = G::new();

    for line in stdin {
        let (item, ingredients) = line.split_once(':').unwrap();

        for ingredient in ingredients.split(',') {
            g.entry(item).or_default().push(ingredient);
            revg.entry(ingredient).or_default().push(item);
        }
    }

    (g, revg)
}

fn bfs<'a>(g: &'a G, source: &'a str) -> BTreeSet<&'a str> {
    let mut seen = BTreeSet::from([source]);

    let mut q = VecDeque::from([source]);
    while let Some(node) = q.pop_front() {
        for &neigh in g.get(node).into_iter().flatten() {
            if seen.insert(neigh) {
                q.push_back(neigh);
            }
        }
    }
    seen
}
