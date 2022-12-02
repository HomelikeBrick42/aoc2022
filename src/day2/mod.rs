fn get_score(a: char, b: char) -> usize {
    (match (a, b) {
        ('A', 'X') => 3,
        ('A', 'Y') => 6,
        ('A', 'Z') => 0,
        ('B', 'X') => 0,
        ('B', 'Y') => 3,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,
        ('C', 'Y') => 0,
        ('C', 'Z') => 3,
        _ => unreachable!(),
    }) + match b {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => unreachable!(),
    }
}

pub fn part1(input: &str) -> usize {
    let mut score = 0;
    for line in input.lines() {
        let (a, b) = line.split_once(' ').unwrap();
        let (a, b) = (
            a.trim().chars().next().unwrap(),
            b.trim().chars().next().unwrap(),
        );
        score += get_score(a, b);
    }
    score
}

pub fn part2(input: &str) -> usize {
    let mut score = 0;
    for line in input.lines() {
        let (a, b) = line.split_once(' ').unwrap();
        let (a, b) = (
            a.trim().chars().next().unwrap(),
            b.trim().chars().next().unwrap(),
        );
        let chosen = match (a, b) {
            ('A', 'X') => 'Z',
            ('A', 'Y') => 'X',
            ('A', 'Z') => 'Y',
            ('B', 'X') => 'X',
            ('B', 'Y') => 'Y',
            ('B', 'Z') => 'Z',
            ('C', 'X') => 'Y',
            ('C', 'Y') => 'Z',
            ('C', 'Z') => 'X',
            _ => unreachable!(),
        };
        score += get_score(a, chosen);
    }
    score
}

#[cfg(test)]
mod test {
    #[test]
    fn part1_test_data() {
        let result = super::part1(include_str!("./test_input.txt"));
        assert_eq!(result, 15);
    }

    #[test]
    fn part1() {
        let result = super::part1(include_str!("./input.txt"));
        assert_eq!(result, 10595);
    }

    #[test]
    fn part2_test_data() {
        let result = super::part2(include_str!("./test_input.txt"));
        assert_eq!(result, 12);
    }

    #[test]
    fn part2() {
        let result = super::part2(include_str!("./input.txt"));
        assert_eq!(result, 9541);
    }
}
