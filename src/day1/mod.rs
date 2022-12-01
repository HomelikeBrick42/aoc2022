pub fn part1(input: &str) -> usize {
    let mut current = 0;
    let mut max = 0;
    for line in input.split(|c| c == '\n') {
        let line = line.trim();
        if line.len() == 0 {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            current += line.parse::<usize>().unwrap();
        }
    }
    max
}

pub fn part2(input: &str) -> usize {
    let mut current = 0;
    let mut max = [0; 3];
    for line in input.split(|c| c == '\n') {
        let line = line.trim();
        if line.len() == 0 {
            let least = max.iter_mut().min().unwrap();
            if current > *least {
                *least = current;
            }
            current = 0;
        } else {
            current += line.parse::<usize>().unwrap();
        }
    }
    max.into_iter().sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn part1_test_data() {
        let result = super::part1(include_str!("./test_input.txt"));
        assert_eq!(result, 24000);
    }

    #[test]
    fn part1() {
        let result = super::part1(include_str!("./input.txt"));
        assert_eq!(result, 72718);
    }

    #[test]
    fn part2_test_data() {
        let result = super::part2(include_str!("./test_input.txt"));
        assert_eq!(result, 45000);
    }

    #[test]
    fn part2() {
        let result = super::part2(include_str!("./input.txt"));
        assert_eq!(result, 213089);
    }
}
