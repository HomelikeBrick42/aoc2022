pub fn part1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let line = line.trim();
        let count = line.chars().count();
        let char = line
            .chars()
            .find_map(|a| {
                line.chars()
                    .skip(count / 2)
                    .find_map(|b| (a == b).then_some(a))
            })
            .unwrap();
        sum += if char.is_uppercase() {
            char as usize - 'A' as usize + 27
        } else {
            char as usize - 'a' as usize + 1
        };
    }
    sum
}

pub fn part2(input: &str) -> usize {
    let mut sum = 0;
    let mut groups = vec![];
    {
        let mut lines = input.lines();
        while let Some(first) = lines.next() {
            groups.push([
                first.trim(),
                lines.next().unwrap().trim(),
                lines.next().unwrap().trim(),
            ]);
        }
    }
    for group in groups {
        let char = group[0]
            .chars()
            .find_map(|a| {
                group[1].chars().find_map(|b| {
                    group[2]
                        .chars()
                        .find_map(|c| (a == b && b == c).then_some(a))
                })
            })
            .unwrap();
        sum += if char.is_uppercase() {
            char as usize - 'A' as usize + 27
        } else {
            char as usize - 'a' as usize + 1
        };
    }
    sum
}

#[cfg(test)]
mod test {
    #[test]
    fn part1_test_data() {
        let result = super::part1(include_str!("./test_input.txt"));
        assert_eq!(result, 157);
    }

    #[test]
    fn part1() {
        let result = super::part1(include_str!("./input.txt"));
        assert_eq!(result, 7568);
    }

    #[test]
    fn part2_test_data() {
        let result = super::part2(include_str!("./test_input.txt"));
        assert_eq!(result, 70);
    }

    #[test]
    fn part2() {
        let result = super::part2(include_str!("./input.txt"));
        assert_eq!(result, 2780);
    }
}
