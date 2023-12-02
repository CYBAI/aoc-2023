use std::collections::HashMap;

use aoc_utils::read_file;
use lazy_regex::regex;

fn main() -> Result<(), ()> {
    let input = read_file("day02/src/input")?;
    let games = parse(&input);

    println!("Part 1: {}", part1(&games));

    Ok(())
}

type GameID = u32;

#[derive(Debug)]
struct Game {
    id: GameID,
    r: u32,
    g: u32,
    b: u32,
}

impl Game {
    fn new(id: u32) -> Self {
        Self {
            id,
            r: 0,
            g: 0,
            b: 0,
        }
    }
}

fn parse(input: &str) -> HashMap<GameID, Game> {
    let mut games = HashMap::new();

    let r = regex!(r"Game (\d+): (.+)");

    for line in input.lines() {
        let matched = r.captures(line).unwrap();

        let id = matched.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let mut game = Game::new(id);

        for set in matched.get(2).unwrap().as_str().split("; ") {
            for cube in set.split(", ") {
                let mut iter = cube.split(" ");

                let count = iter.next().unwrap().parse::<u32>().unwrap();
                let color = iter.next().unwrap();

                match color {
                    "red" => {
                        game.r = count.max(game.r);
                    }
                    "green" => {
                        game.g = count.max(game.g);
                    }
                    "blue" => {
                        game.b = count.max(game.b);
                    }
                    _ => panic!("Unknown color: {}", color),
                }
            }
        }

        games.insert(id, game);
    }

    games
}

fn part1(games: &HashMap<GameID, Game>) -> u32 {
    games
        .iter()
        .filter_map(|(id, game)| {
            if game.r <= 12 && game.g <= 13 && game.b <= 14 {
                Some(id)
            } else {
                None
            }
        })
        .sum()
}

#[test]
fn test_part1() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let games = parse(input);
    assert_eq!(part1(&games), 8);
}
