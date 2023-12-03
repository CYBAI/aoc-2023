use std::collections::HashMap;

use aoc_utils::read_file;

type Coordinate = (usize, usize);

#[derive(Debug)]
struct Number {
    value: u32,
    is_part: bool,
}

#[derive(Debug)]
struct Board {
    numbers: Vec<Number>,
}

fn main() -> Result<(), ()> {
    let input = read_file("day03/src/input")?;
    let board = parse(&input);

    println!("Part 1: {}", part1(&board));

    Ok(())
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn parse(input: &str) -> Board {
    let mut numbers = Vec::new();
    let mut symbols = HashMap::new();
    let height = input.lines().count();
    let width = input.lines().next().map(|l| l.trim().len()).unwrap_or(0);

    let mut raw_board = vec![vec!['.'; width]; height];

    for (y, line) in input.lines().enumerate() {
        let line = line.trim();

        for (x, c) in line.chars().enumerate() {
            raw_board[y][x] = c;

            if is_symbol(c) {
                symbols.insert((x, y), c);
            }
        }
    }

    for y in 0..height {
        let mut curr_num: Option<(u32, bool)> = None;

        for x in 0..width {
            let coord = (x, y);
            let c = raw_board[y][x];

            if let Some(n) = c.to_digit(10) {
                let siblings = generate_siblings(&coord, &(width, height));
                let is_part = siblings.iter().any(|coord| symbols.contains_key(&coord));

                match curr_num {
                    Some((p, prev_b)) => {
                        curr_num = Some((
                            p * 10 + n,
                            prev_b || is_part,
                        ));
                    }
                    None => {
                        curr_num = Some((n, is_part));
                    }
                }
            }

            if !c.is_digit(10) || x == width - 1 {
                if let Some((value, is_part)) = curr_num {
                    numbers.push(Number { value, is_part });

                    curr_num = None;
                }
            }
        }
    }

    Board { numbers }
}

fn part1(board: &Board) -> u32 {
    board
        .numbers
        .iter()
        .filter_map(|num| if num.is_part { Some(num.value) } else { None })
        .sum()
}

fn generate_siblings(coord: &Coordinate, boundary: &Coordinate) -> Vec<Coordinate> {
    let (x, y) = *coord;
    let (bx, by) = *boundary;

    let x = x as isize;
    let y = y as isize;

    vec![
        (x + 1, y),
        (x + 1, y + 1),
        (x + 1, y - 1),
        (x, y + 1),
        (x, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x - 1, y - 1),
    ]
    .iter()
    .filter_map(|(x, y)| {
        if 0 <= *x && (*x as usize) < bx && 0 <= *y && (*y as usize) < by {
            Some((*x as usize, *y as usize))
        } else {
            None
        }
    })
    .collect::<Vec<_>>()
}

#[test]
fn test_part1() {
    let input = "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    let board = parse(input);
    assert_eq!(part1(&board), 4361);
}
