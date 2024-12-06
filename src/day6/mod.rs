enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn turn(d: Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

fn vector_of_dir(d: &Direction) -> (i32, i32) {
    match d {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    }
}

fn next_pos(p: (i32, i32), d: &Direction) -> (i32, i32) {
    let (x, y) = p;
    let (dx, dy) = vector_of_dir(d);

    (x + dx, y + dy)
}

fn is_inside(t: &[Vec<Option<i32>>], p: (i32, i32)) -> bool {
    let (x, y) = p;
    y >= 0 && y < t.len() as i32 && x >= 0 && x < t[y as usize].len() as i32
}

fn is_wall(t: &[Vec<Option<i32>>], p: (i32, i32)) -> bool {
    let (x, y) = p;
    is_inside(t, p) && t[x as usize][y as usize].is_none()
}

pub fn part1() -> String {
    let path = "input/day6.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let mut board: Vec<Vec<Option<i32>>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '.' {
                        Some(0)
                    } else if c == '#' {
                        None
                    } else if c == '^' {
                        Some(1)
                    } else {
                        unreachable!()
                    }
                })
                .collect()
        })
        .collect();

    let mut pos = (0, 0);

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == Some(1) {
                pos = (i as i32, j as i32);
                break;
            }
        }
    }

    let mut d = Direction::Up;

    while is_inside(&board, pos) {
        let n = next_pos(pos, &d);
        if is_wall(&board, n) {
            d = turn(d);
        } else {
            board[pos.0 as usize][pos.1 as usize] = Some(1);
            pos = n;
        }
    }

    board
        .iter()
        .flatten()
        .filter_map(|v| *v)
        .sum::<i32>()
        .to_string()
}

pub fn part2() -> String {
    let path = "input/day6.txt";
    let input = std::fs::read_to_string(path).unwrap();

    "".to_string()
}
