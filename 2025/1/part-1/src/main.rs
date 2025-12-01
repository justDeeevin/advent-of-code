const INPUT: &str = include_str!("../../input.txt");

fn main() {
    let mut n = 50;
    let mut out = 0;
    for delta in INPUT.lines().map(parse) {
        dbg!(delta);
        n = (n + delta) % 100;
        dbg!(n);
        if n == 0 {
            out += 1;
        }
    }

    println!("{out}");
}

fn parse(line: &str) -> i16 {
    dbg!(line);
    let direction = &line[..1];
    let distance = line[1..].parse().unwrap();
    match direction {
        "R" => distance,
        "L" => -distance,
        _ => unreachable!(),
    }
}
