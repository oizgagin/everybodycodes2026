use std::{
    cmp::max,
    collections::VecDeque,
    io::{self, Read, Write},
};

fn main() {
    let mut stdout = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf).unwrap();

    let groups = parse(buf);
    solve1(&mut stdout, &groups);
}

fn parse(input: String) -> Vec<Vec<(usize, u8, usize)>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|order| {
                    let order = order.as_bytes();
                    (
                        (order[0] - b'0') as usize,
                        order[1],
                        (order[2] - b'0') as usize,
                    )
                })
                .collect()
        })
        .collect()
}

fn solve1<W>(w: &mut W, groups: &[Vec<(usize, u8, usize)>])
where
    W: Write,
{
    let mut res = 0;

    for group in groups {
        let max = group
            .iter()
            .map(|&(alloy1, _, alloy2)| max(alloy1, alloy2))
            .max()
            .unwrap();

        let mut g = vec![Vec::new(); max + 1];
        let mut indeg = vec![0; max + 1];

        for &(alloy1, op, alloy2) in group {
            let (mut u, mut v) = (alloy1, alloy2);
            if op == b'<' {
                (u, v) = (alloy2, alloy1);
            }

            g[u].push(v);
            indeg[v] += 1;
        }

        let mut q = VecDeque::new();
        for u in 1..=max {
            if indeg[u] == 0 {
                q.push_back(u);
            }
        }

        let mut order = Vec::new();

        while let Some(u) = q.pop_front() {
            order.push(u);

            for &v in &g[u] {
                indeg[v] -= 1;
                if indeg[v] == 0 {
                    q.push_back(v);
                }
            }
        }

        let mut checksum = 0;
        for &digit in &order {
            checksum = checksum * 10 + digit;
        }

        res += checksum;
    }

    write!(w, "{res}").unwrap();
}
