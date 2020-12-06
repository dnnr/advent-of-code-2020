use std::collections::HashSet;

pub fn part1(inp: String) {
    let groups = read_groups(&inp);

    let answers = groups.iter().map(|group| get_answers_for_group(group));

    let sum: usize = answers.map(|a| a.len()).sum();

    println!("sum: {:?}", sum);
}

pub fn part2(_inp: String) {}

fn read_groups(inp: &str) -> Vec<&str> {
    inp.split("\n\n")
        .filter(|line| line.len() > 0)
        .collect::<Vec<&str>>()
}

fn get_answers_for_group(group: &str) -> HashSet<char> {
    group
        .chars()
        .filter(|c| match c {
            'a'..='z' => true,
            _ => false,
        })
        .collect::<HashSet<char>>()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    pub fn get_answers_for_group_sample1() {
        let group = "abcx\nabcy\nabcz\n";
        let expected_answers = vec!['a', 'b', 'c', 'x', 'y', 'z'];

        let answers = get_answers_for_group(group);

        assert_eq!(
            answers,
            HashSet::from_iter(expected_answers.iter().cloned())
        );
    }
}
