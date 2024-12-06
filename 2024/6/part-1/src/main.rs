#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Square {
    Empty { crossed: bool },
    Wall,
    Guard { direction: Direction },
}

impl TryFrom<char> for Square {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Square::Empty { crossed: false }),
            'X' => Ok(Square::Empty { crossed: true }),
            '#' => Ok(Square::Wall),
            '^' => Ok(Square::Guard {
                direction: Direction::Up,
            }),
            'v' => Ok(Square::Guard {
                direction: Direction::Down,
            }),
            '>' => Ok(Square::Guard {
                direction: Direction::Right,
            }),
            '<' => Ok(Square::Guard {
                direction: Direction::Left,
            }),
            _ => Err(()),
        }
    }
}

impl From<Square> for char {
    fn from(value: Square) -> char {
        match value {
            Square::Empty { crossed: false } => '.',
            Square::Empty { crossed: true } => 'X',
            Square::Wall => '#',
            Square::Guard {
                direction: Direction::Up,
            } => '^',
            Square::Guard {
                direction: Direction::Down,
            } => 'v',
            Square::Guard {
                direction: Direction::Right,
            } => '>',
            Square::Guard {
                direction: Direction::Left,
            } => '<',
        }
    }
}

impl std::fmt::Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

fn main() {
    let mut guard_coords = None;
    let mut grid = include_str!("../../input.txt")
        .lines()
        .enumerate()
        .map(|(left, l)| {
            l.chars()
                .enumerate()
                .map(|(right, c)| {
                    let out = Square::try_from(c).unwrap();
                    if matches!(out, Square::Guard { direction: _ }) {
                        guard_coords = Some((left, right));
                    }
                    out
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let Some(mut guard_coords) = guard_coords else {
        panic!("No guard found");
    };

    loop {
        let Square::Guard { direction } = grid[guard_coords.0][guard_coords.1] else {
            panic!("Incorrect guard coords");
        };
        if (guard_coords.0 == 0 && direction == Direction::Up)
            || (guard_coords.0 == grid.len() - 1 && direction == Direction::Down)
            || (guard_coords.1 == 0 && direction == Direction::Left)
            || (guard_coords.1 == grid[0].len() - 1 && direction == Direction::Right)
        {
            break;
        }
        match direction {
            Direction::Up => {
                if grid[guard_coords.0 - 1][guard_coords.1] == Square::Wall {
                    grid[guard_coords.0][guard_coords.1] = Square::Guard {
                        direction: Direction::Right,
                    };
                } else {
                    grid[guard_coords.0][guard_coords.1] = Square::Empty { crossed: true };
                    guard_coords.0 -= 1;
                    grid[guard_coords.0][guard_coords.1] = Square::Guard { direction };
                }
            }
            Direction::Down => {
                if grid[guard_coords.0 + 1][guard_coords.1] == Square::Wall {
                    grid[guard_coords.0][guard_coords.1] = Square::Guard {
                        direction: Direction::Left,
                    };
                } else {
                    grid[guard_coords.0][guard_coords.1] = Square::Empty { crossed: true };
                    guard_coords.0 += 1;
                    grid[guard_coords.0][guard_coords.1] = Square::Guard { direction };
                }
            }
            Direction::Right => {
                if grid[guard_coords.0][guard_coords.1 + 1] == Square::Wall {
                    grid[guard_coords.0][guard_coords.1] = Square::Guard {
                        direction: Direction::Down,
                    };
                } else {
                    grid[guard_coords.0][guard_coords.1] = Square::Empty { crossed: true };
                    guard_coords.1 += 1;
                    grid[guard_coords.0][guard_coords.1] = Square::Guard { direction };
                }
            }
            Direction::Left => {
                if grid[guard_coords.0][guard_coords.1 - 1] == Square::Wall {
                    grid[guard_coords.0][guard_coords.1] = Square::Guard {
                        direction: Direction::Up,
                    };
                } else {
                    grid[guard_coords.0][guard_coords.1] = Square::Empty { crossed: true };
                    guard_coords.1 -= 1;
                    grid[guard_coords.0][guard_coords.1] = Square::Guard { direction };
                }
            }
        }
    }
    // Starting at 1 because the final X isn't added with this method
    let out = grid.iter().flatten().fold(1, |acc, square| {
        if 'X' == (*square).into() {
            acc + 1
        } else {
            acc
        }
    });

    println!("{}", out);
}

fn print_grid(grid: &Vec<Vec<Square>>) {
    for row in grid {
        println!("{:?}", row);
    }
}
