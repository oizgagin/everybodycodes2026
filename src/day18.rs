use std::{
    cmp::{max, min},
    collections::BTreeMap,
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
    let ys = parse1(stdin);

    let xs: Vec<usize> = ys.keys().copied().collect();
    let (minx, maxx) = (*xs.iter().min().unwrap(), *xs.iter().max().unwrap());

    let mut res = 0;

    let mut maxl = ys[&minx];
    let mut maxr = ys[&maxx];

    let (mut i, mut j) = (minx, maxx);
    while i <= j {
        let ceil = min(maxl, maxr);
        let (yi, yj) = (*ys.get(&i).unwrap_or(&0), *ys.get(&j).unwrap_or(&0));

        if yi < yj {
            res += yi + ceil.saturating_sub(yi);
            i += 1;
        } else {
            res += yj + ceil.saturating_sub(yj);
            j -= 1;
        }

        maxr = max(maxr, yj);
        maxl = max(maxl, yi);
    }

    writeln!(stdout, "{res}").unwrap();
}

fn parse1(s: &str) -> BTreeMap<usize, usize> {
    BTreeMap::from_iter(
        s.lines()
            .flat_map(|l| l.split_once(' '))
            .map(|(x, y)| (x.strip_prefix("x=").unwrap(), y.strip_prefix("y=").unwrap()))
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap())),
    )
}

fn solve2<W>(stdin: &str, stdout: &mut W)
where
    W: Write,
{
    let ys = parse2(stdin);

    let xs: Vec<usize> = ys.keys().copied().collect();
    let (minx, maxx) = (*xs.iter().min().unwrap(), *xs.iter().max().unwrap());

    let mut res = 0;

    let mut maxl = ys[&minx];
    let mut maxr = ys[&maxx];

    let (mut i, mut j) = (minx, maxx);
    while i <= j {
        let ceil = min(maxl, maxr);
        let (yi, yj) = (*ys.get(&i).unwrap_or(&0), *ys.get(&j).unwrap_or(&0));

        if yi < yj {
            res += yi + ceil.saturating_sub(yi);
            i += 1;
        } else {
            res += yj + ceil.saturating_sub(yj);
            j -= 1;
        }

        maxr = max(maxr, yj);
        maxl = max(maxl, yi);
    }

    writeln!(stdout, "{res}").unwrap();
}

fn parse2(s: &str) -> BTreeMap<usize, usize> {
    BTreeMap::from_iter(
        s.lines()
            .map(|l| l.split_ascii_whitespace().collect::<Vec<&str>>())
            .map(|v| {
                (
                    v[0].strip_prefix("x=").unwrap(),
                    v[1].strip_prefix("y=").unwrap(),
                    v[2].strip_prefix("width=").unwrap(),
                )
            })
            .flat_map(|(x, y, w)| {
                let w: usize = w.parse().unwrap();
                (0..w).map(|d| (x.parse::<usize>().unwrap() + d, y.parse::<usize>().unwrap()))
            }),
    )
}
