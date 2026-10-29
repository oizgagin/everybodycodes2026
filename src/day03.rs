use std::io::{self, Read, Write};

fn main() {
    let mut stdout = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf).unwrap();

    let votes = parse(buf);
    //solve1(&mut stdout, &votes);
    //solve2(&mut stdout, &votes);
    solve3(&mut stdout, &votes);
}

fn parse(input: String) -> Vec<(Vec<u64>, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let (iso, votes) = line.split_once(' ').unwrap();

            (
                iso.strip_prefix("ISO2-")
                    .unwrap()
                    .bytes()
                    .map(|b| (b - b'0') as u64)
                    .collect(),
                votes.bytes().map(|b| (b - b'0') as u64).collect(),
            )
        })
        .collect()
}

fn solve1<W>(w: &mut W, votes: &[(Vec<u64>, Vec<u64>)])
where
    W: Write,
{
    let res = votes
        .iter()
        .map(|(iso, votes)| {
            iso.iter()
                .rev()
                .enumerate()
                .zip(votes.iter().rev())
                .map(|((pow, b), d)| d * b.pow(pow as u32))
                .sum::<u64>()
        })
        .sum::<u64>();

    writeln!(w, "{res}").unwrap();
}

fn solve2<W>(w: &mut W, votes: &[(Vec<u64>, Vec<u64>)])
where
    W: Write,
{
    let res = votes
        .iter()
        .map(|(iso, votes)| {
            iso.iter()
                .rev()
                .cycle()
                .enumerate()
                .zip(votes.iter().rev())
                .map(|((pow, b), d)| d * b.pow(pow as u32))
                .sum::<u64>()
        })
        .sum::<u64>();

    writeln!(w, "{res}").unwrap();
}

fn solve3<W>(w: &mut W, votes: &[(Vec<u64>, Vec<u64>)])
where
    W: Write,
{
    fn to_digits(mut x: u64, buf: &mut Vec<u64>) {
        buf.clear();

        if x == 0 {
            buf.push(0);
            return;
        }

        while x != 0 {
            buf.push(x % 10);
            x /= 10;
        }
    }

    fn to_num(digits: &Vec<u64>) -> u64 {
        digits.iter().fold(0, |acc, &d| acc * 10 + d)
    }

    fn eval(iso: &Vec<u64>, vote: u64, buf: &mut Vec<u64>) -> u64 {
        to_digits(vote, buf);

        iso.iter()
            .rev()
            .cycle()
            .enumerate()
            .zip(buf.iter())
            .map(|((pow, b), d)| d * b.pow(pow as u32))
            .sum::<u64>()
    }

    let mut res = 0;

    let mut buf = Vec::with_capacity(20);

    for (iso, votes) in votes {
        let curr_num = to_num(votes);
        let curr_res = eval(iso, curr_num, &mut buf);

        let mut valid = true;
        for i in 0..curr_num {
            if eval(iso, i, &mut buf) == curr_res {
                valid = false;
                break;
            }
        }

        if valid {
            res += curr_res
        }
    }

    writeln!(w, "{res}").unwrap();
}
