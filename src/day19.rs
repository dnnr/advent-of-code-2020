use std::collections::HashMap;
use std::collections::HashSet;

pub fn part1(inp: String) {
    let (rules, words) = read_input(&inp);
    let rules = parse_rules(&rules);

    let generated_words = generate_words(&rules);

    let words: HashSet<_> = words.iter().map(|x| x.to_string()).collect();

    let same = generated_words.union(&words).count();

    println!("Matches: {}", same);
}

pub fn part2(_inp: String) {
}

#[derive(Debug)]
enum Rhs {
    Terminal(char),
    MoreRules(Vec<Vec<usize>>),
}

// -1: 0

// queue: [[0]]
// queue: [[1,2]]
// queue: [[1,1,3], [1,3,1]]


// queue: [[1,2,2]]


fn generate_words(rules: &HashMap<usize, Rhs>) -> HashSet<String> {
    let mut finished_words: HashSet<Vec<char>> = HashSet::new();
    // let mut queue = rules.get(&0).unwrap();
    let mut queue: HashSet<Vec<usize>> = match rules.get(&0).unwrap() {
        Rhs::MoreRules(x) => x.iter().cloned().collect(),
        _ => panic!("wut?")
    };

    let mut queue_seen: HashSet<Vec<usize>> = match rules.get(&0).unwrap() {
        Rhs::MoreRules(x) => x.iter().cloned().collect(),
        _ => panic!("wut?")
    };

    while queue.len() > 0 {
        let entry = queue.iter().next().cloned().unwrap();
        let entry = queue.take(&entry).unwrap();
        // let entry = queue.pop().unwrap(); // -> entry is Vec<usize>

        // Is it all terminals?
        let is_all_terminals = entry.iter().all(|x| // -> x is usize
            match rules.get(x).unwrap() {
                Rhs::Terminal(_) => true,
                Rhs::MoreRules(_) => false,
            });

        if is_all_terminals {
            // convert it and move it to finished words
            let word: Vec<char> = entry.iter().map(|x|
                match rules.get(x).unwrap() {
                    Rhs::Terminal(character) => *character,
                    _ => panic!("wuut?"),
                }).collect();
            finished_words.insert(word);
        } else {
            // look for first non-terminal, generate 1 or 2 new queue entries from it
            // entry.iter().enumerate().
            for (index, value) in entry.iter().enumerate() {
                match rules.get(value).unwrap() {
                    Rhs::Terminal(_) => {},
                    Rhs::MoreRules(new_rhs) => {
                        for variant in new_rhs {
                            let mut foo: Vec<usize> = entry[0..index].to_vec();
                            for x in variant {
                                foo.push(*x);
                            }
                            for x in entry[index+1..entry.len()].to_vec() {
                                foo.push(x);
                            }
                            if !queue_seen.contains(&foo) {
                                queue_seen.insert(foo.clone());
                                queue.insert(foo);
                            }
                        }
                    }
                };
            }
        }
        println!("Queue length: {}, queue seen: {}, finished words: {}", queue.len(), queue_seen.len(), finished_words.len());
        // println!("Queue (q:{}, f:{}): {:?}", queue.len(), finished_words.len(), queue);
    }

    finished_words.iter().map(|x| x.into_iter().collect()).collect()
}

fn parse_rules(rules: &Vec<&str>) -> HashMap<usize, Rhs> {
    rules.iter()
        .map(|line| parse_rule_line(&line))
        .collect::<HashMap<usize, Rhs>>()
}

fn parse_rule_line(line: &str) -> (usize, Rhs) {
    let parts = line.split(": ").collect::<Vec<&str>>();
    let lhs = parts[0].parse::<usize>().unwrap();
    let rhs = parts[1];

    if rhs == "\"a\"" {
        return (lhs, Rhs::Terminal('a'));
    } else if rhs == "\"b\"" {
        return (lhs, Rhs::Terminal('b'));
    } else {
        let morerules = Rhs::MoreRules(
            rhs.split(" | ")
            .map(|variant| variant
                .split(" ")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>())
            .collect::<Vec<Vec<usize>>>());
        return (lhs, morerules);
    }
}

fn read_input(inp: &str) -> (Vec<&str>, Vec<&str>) {
    let sections = inp.split("\n\n").collect::<Vec<&str>>();
    let rules = sections[0].split("\n")
        .filter(|line| line.len() > 0)
        .collect::<Vec<_>>();

    let words = sections[1].split("\n")
        .filter(|line| line.len() > 0)
        .collect::<Vec<_>>();

    (rules, words)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_to_generate_words() {
        let rules_strs = vec![
            "0: 1 2",
            "1: \"a\"",
            "2: 1 3 | 3 1",
            "3: \"b\"",
        ];
        let rules = parse_rules(&rules_strs);

        let words = generate_words(&rules);

        let expected_words: HashSet<_> = [
            "aba".to_string(),
            "aab".to_string(),
        ].iter().cloned().collect();
        assert_eq!(words, expected_words);
    }

    #[test]
    fn try_to_generate_words2() {
        let rules_strs = vec![
            "0: 4 1 5",
            "1: 2 3 | 3 2",
            "2: 4 4 | 5 5",
            "3: 4 5 | 5 4",
            "4: \"a\"",
            "5: \"b\"",
        ];
        let rules = parse_rules(&rules_strs);

        let words = generate_words(&rules);

        let expected_words: HashSet<_> = [
            "aaaabb".to_string(),
            "aaabab".to_string(),
            "abbabb".to_string(),
            "abbbab".to_string(),
            "aabaab".to_string(),
            "aabbbb".to_string(),
            "abaaab".to_string(),
            "ababbb".to_string(),
        ].iter().cloned().collect();
        assert_eq!(words, expected_words);
    }
}
