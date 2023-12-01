use aoc_utils::read_file;

fn main() -> Result<(), ()> {
    let input = read_file("day01/src/input")?;

    println!("Part 1: {}", part1(&input));

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

#[test]
fn test_part1() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    assert_eq!(part1(input), 142);
}
