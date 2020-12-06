use std::collections::HashSet;
use std::iter::FromIterator;

pub fn part1(inp: String) {
    let groups = read_groups(&inp);

    let any_answers = groups.iter().map(|group| get_any_answers_in_group(group));

    let sum: usize = any_answers.map(|a| a.len()).sum();

    println!("part1: {:?}", sum);
}

pub fn part2(inp: String) {
    let groups = read_groups(&inp);

    let common_answers = groups.iter().map(|group| get_common_answers_in_group(group));

    let sum: usize = common_answers.map(|a| a.len()).sum();

    println!("part2: {:?}", sum);
}

fn read_groups(inp: &str) -> Vec<&str> {
    inp.split("\n\n")
        .filter(|line| line.len() > 0)
        .collect::<Vec<&str>>()
}

fn get_common_answers_in_group(group: &str) -> HashSet<char> {
    let people = group
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| HashSet::from_iter(line.chars()))
        .collect::<Vec<HashSet<char>>>();

    if people.is_empty() {
        return HashSet::new();
    }

    let mut people_iter = people.iter();
    let mut common_answers = people_iter.next().unwrap().clone();
    people_iter.for_each(|answers| common_answers.retain(|x| answers.contains(x)));

    common_answers
}

fn get_any_answers_in_group(group: &str) -> HashSet<char> {
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
    pub fn get_any_answers_in_group_sample1() {
        let group = "abcx\nabcy\nabcz\n";
        let expected_answers = vec!['a', 'b', 'c', 'x', 'y', 'z'];

        let answers = get_any_answers_in_group(group);

        assert_eq!(
            answers,
            HashSet::from_iter(expected_answers.iter().cloned())
        );
    }


    #[test]
    pub fn get_common_answers_in_group_sample1() {
        let group = "abcx\nabcy\nabcz\n";
        let expected_answers = vec!['a', 'b', 'c'];

        let answers = get_common_answers_in_group(group);

        assert_eq!(
            answers,
            HashSet::from_iter(expected_answers.iter().cloned())
        );
    }
}
