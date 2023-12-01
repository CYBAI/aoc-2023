use aoc_utils::read_file;

fn main() -> Result<(), ()> {
    let input = read_file("day01/src/input")?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut filtered_digits = line.chars().filter_map(|c| c.to_digit(10));

            let head = filtered_digits.next().unwrap();
            let tail = filtered_digits.last().unwrap_or(head);

            head * 10 + tail
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut head = None;
            let mut tail = None;

            for i in 0..line.len() {
                let n = line
                    .chars()
                    .nth(i)
                    .and_then(|n| n.to_digit(10))
                    .or_else(|| {
                        let mut n = None;

                        if i + 3 <= line.len() {
                            match &line[i..i + 3] {
                                "one" => n = Some(1),
                                "two" => n = Some(2),
                                "six" => n = Some(6),
                                _ => {}
                            }
                        }

                        if i + 4 <= line.len() {
                            match &line[i..i + 4] {
                                "four" => n = Some(4),
                                "five" => n = Some(5),
                                "nine" => n = Some(9),
                                _ => {}
                            }
                        }

                        if i + 5 <= line.len() {
                            match &line[i..i + 5] {
                                "three" => n = Some(3),
                                "seven" => n = Some(7),
                                "eight" => n = Some(8),
                                _ => {}
                            }
                        }

                        n
                    });

                if n.is_some() {
                    if head.is_none() {
                        head = n;
                    } else {
                        tail = n;
                    }
                }
            }

            let head = head.unwrap();

            head * 10 + tail.unwrap_or(head)
        })
        .sum()
}

#[test]
fn test_part1() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    assert_eq!(part1(input), 142);
}

#[test]
fn test_part2() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    assert_eq!(part2(input), 281);
}
