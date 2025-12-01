const INPUT: &str = include_str!("../../input.txt");
/// The number of marks on the dial
const PIPS: N = 100;

type N = i16;

fn main() {
    let mut n: u8 = 50;
    let mut out = 0;

    for delta in INPUT.lines().map(parse) {
        let loops = delta.abs() / PIPS;
        out += loops;
        if (delta < 0 && delta.abs() % PIPS >= n as N)
            || (delta > 0 && delta % PIPS + n as N >= PIPS)
        {
            out += 1;
        }
        n = ((n as N + delta) % PIPS) as u8;
    }

    println!("{out}");
}

fn parse(line: &str) -> N {
    let direction = &line[..1];
    let distance = line[1..].parse().unwrap();
    match direction {
        "R" => distance,
        "L" => -distance,
        _ => unreachable!(),
    }
}
