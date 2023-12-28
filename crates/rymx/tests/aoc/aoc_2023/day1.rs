pub fn main() {
    let input = include_str!("./day1.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|line| {
            let left = line.chars().find_map(|c| c.to_digit(10))?;
            let right = line.chars().rev().find_map(|c| c.to_digit(10))?;
            Some(left * 10 + right)
        })
        .sum::<u32>()
}

fn part_2(input: &str) -> u32 {
    let digit_strings = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    input
        .lines()
        .flat_map(|line| {
            let mut iter = (0..line.len()).filter_map(|pos| {
                let substr = &line[pos..];

                if let Some(digit) = substr.chars().next().and_then(|c| c.to_digit(10)) {
                    return Some(digit);
                }

                digit_strings.into_iter().fold(None, |_, (string, digit)| {
                    if substr.starts_with(string) {
                        Some(digit)
                    } else {
                        None
                    }
                })
            });
            let left = iter.next()?;
            let right = iter.last().unwrap_or(left);
            Some(left * 10 + right)
        })
        .sum()
}
