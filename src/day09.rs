pub fn part1(inp: String) {
    let preamble_length = 25;
    let numbers = read_numbers(&inp);

    let first_invalid = find_first_invalid_number(&numbers, preamble_length);

    println!("First invalid number: {}", first_invalid);
}

pub fn part2(_inp: String) {}

fn find_first_invalid_number(numbers: &Vec<usize>, preamble_length: usize) -> usize {
    for (index, number) in numbers.iter().enumerate().skip(preamble_length) {
        let preamble = get_preamble(&numbers, index, preamble_length);
        if !is_valid_number(&preamble, *number) {
            return *number;
        }
    }
    panic!("No invalid number found");
}

fn read_numbers(inp: &str) -> Vec<usize> {
    inp.split("\n")
        .filter(|line| line.len() > 0)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn is_valid_number(preamble: &Vec<usize>, number: usize) -> bool {
    for a in preamble {
        for b in preamble {
            if a != b && (a + b) == number {
                return true;
            }
        }
    }
    false
}

fn get_preamble(all_numbers: &Vec<usize>, index: usize, preamble_length: usize) -> Vec<usize> {
    let from = index - preamble_length;
    let to = index;
    // TODO: Can we return a slice here instead of creating a new Vec?
    all_numbers[from..to].to_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn get_premable_sample2() {
        let all_numbers = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let index = 7;
        let length = 3;
        let expected_preamble = vec![5, 4, 3];

        let preamble = get_preamble(&all_numbers, index, length);

        assert_eq!(preamble, expected_preamble);
    }

    #[test]
    pub fn is_valid_number_sample1() {
        let preamble = vec![35, 20, 15, 25, 47];
        let given_value = 40;

        let is_valid = is_valid_number(&preamble, given_value);

        assert_eq!(is_valid, true);
    }

    #[test]
    pub fn is_valid_number_sample2() {
        let preamble = vec![95, 102, 117, 150, 182];
        let given_value = 127;

        let is_valid = is_valid_number(&preamble, given_value);

        assert_eq!(is_valid, false);
    }
}
