use std::{
    cmp::max,
    io::{self, Read, Write},
};

fn main() {
    let mut stdout = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf).unwrap();
    let mut stdin = buf.split_ascii_whitespace();

    let notes: Vec<u32> = stdin
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    solve1(&mut stdout, &notes);
    solve2(&mut stdout, &notes);
    solve3(&mut stdout, &notes);
}

fn solve1<W>(w: &mut W, notes: &[u32])
where
    W: Write,
{
    let mut res = 0;

    for &cones in notes {
        let s = cones.isqrt();
        if s * s == cones {
            continue;
        }
        res += (s + 1) * (s + 1) - cones;
    }

    writeln!(w, "{res}").unwrap();
}

fn solve2<W>(w: &mut W, notes: &[u32])
where
    W: Write,
{
    let mut res = 0;

    for &cones in notes {
        let n = max((cones + 4).div_ceil(4), 3);
        res += 4 * n - 4 - cones;
    }

    writeln!(w, "{res}").unwrap();
}

fn solve3<W>(w: &mut W, notes: &[u32])
where
    W: Write,
{
    /*

    Formation with side n >= 3, n % 2 != 0 requires n*n - 1 cones.

    So for given x, we:

    1. check if (x + 1) is a perfect square and that
       sqrt(x+1) is >= 3; if it is - skip;

    2. otherwise need to find min n >= 3, n % 2 != 0 such that
       n*n - 1 > x; or n*n > x + 1; or n = max(3, (x + 1).isqrt() + 1)

    */

    let mut res = 0;

    for &cones in notes {
        let n = (cones + 1).isqrt();
        if n * n == (cones + 1) && n >= 3 && n % 2 != 0 {
            continue;
        }

        let mut n = max(3, n + 1);
        if n.is_multiple_of(2) {
            n += 1;
        }

        res += n * n - 1 - cones;
    }

    writeln!(w, "{res}").unwrap();
}

#[allow(dead_code)]
mod lib {
    pub fn parse<'a, T>(stdin: &mut impl Iterator<Item = &'a str>) -> Result<T, T::Err>
    where
        T: std::str::FromStr,
    {
        stdin.next().unwrap().parse::<T>()
    }
}
