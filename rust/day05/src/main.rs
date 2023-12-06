use rayon::{slice::ParallelSlice, iter::ParallelIterator};

fn main() {
    let input = include_str!("./input");
    let almanac = Almanac::parse(input);

    println!("Part 1: {}", part1(&almanac));
    println!("Part 2: {}", part2(&almanac));
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    category_maps: Vec<Vec<CategoryMap>>,
}

impl Almanac {
    fn parse(input: &str) -> Almanac {
        let mut groups = input.split("\n\n");
        let seeds = groups
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let mut category_maps = Vec::new();

        for group in groups {
            let mut vecs = Vec::new();

            let line = group.trim();
            let maps = line.split("\n").skip(1).collect::<Vec<_>>();

            for map in maps {
                let nums = map.split_whitespace().collect::<Vec<_>>();

                let dest = nums[0].parse::<u64>().unwrap();
                let source = nums[1].parse::<u64>().unwrap();
                let length = nums[2].parse::<u64>().unwrap();

                vecs.push(CategoryMap {
                    source,
                    dest,
                    length,
                });
            }

            category_maps.push(vecs);
        }

        Almanac {
            seeds,
            category_maps,
        }
    }
}

#[derive(Debug)]
struct CategoryMap {
    source: u64,
    dest: u64,
    length: u64,
}

impl CategoryMap {
    fn convert(&self, target: u64) -> Option<u64> {
        let lower_bound = self.source;
        let upper_bound = self.source + self.length;
        let bounds = lower_bound..=upper_bound;

        if !bounds.contains(&target) {
            return None;
        }

        Some(self.dest + target - self.source)
    }
}

fn part1(almanac: &Almanac) -> u64 {
    almanac
        .seeds
        .iter()
        .map(|seed| find_lowest_location(&almanac.category_maps, seed))
        .min()
        .unwrap()
}

fn part2(almanac: &Almanac) -> u64 {
    almanac
        .seeds
        .par_chunks_exact(2)
        .flat_map(|xs| {
            let f = xs[0];
            let l = xs[1];

            f..(f + l)
        })
        .map(|seed| find_lowest_location(&almanac.category_maps, &seed))
        .min()
        .unwrap()
}

fn find_lowest_location(category_maps: &Vec<Vec<CategoryMap>>, seed: &u64) -> u64 {
    category_maps.iter().fold(*seed, |current_seed, maps| {
        maps.iter()
            .find_map(|map| map.convert(current_seed))
            .unwrap_or(current_seed)
    })
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = r"seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4";

    #[test]
    fn test_part1() {
        let almanac = Almanac::parse(INPUT);
        assert_eq!(part1(&almanac), 35);
    }

    #[test]
    fn test_part2() {
        let almanac = Almanac::parse(INPUT);
        assert_eq!(part2(&almanac), 46);
    }
}
