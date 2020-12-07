use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

pub fn part1(inp: String) {
    let rule_map = make_rule_map(&inp);

    let mut holds_shiny_gold = HashSet::<String>::new();

    for (left_color, _) in rule_map.iter() {
        if shiny_gold_is_reachable(left_color, &rule_map, &holds_shiny_gold) {
            holds_shiny_gold.insert(left_color.clone());
        }
    }

    println!("Count of shiny gold holders: {}", holds_shiny_gold.len());
}

pub fn part2(_inp: String) {
}

fn shiny_gold_is_reachable(color: &String, rule_map: &HashMap<String, Vec<RuleEntry>>, known_true: &HashSet<String>) -> bool {
    if known_true.contains(color) {
        return true;
    }
    for rule_entry in rule_map[color].iter() {
        if shiny_gold_is_reachable(&rule_entry.color, rule_map, known_true) {
            return true;
        }
        if rule_entry.color == "shiny gold" {
            return true;
        }
    }
    false
}

fn make_rule_map(inp: &str) -> HashMap<String, Vec<RuleEntry>> {
    let rule_strings = read_rule_strings(&inp);

    let rule_parser = RuleParser::new();

    rule_strings
        .iter()
        .map(|r| rule_parser.parse(r.to_string()))
        .collect::<HashMap<_, _>>()
}

fn read_rule_strings(inp: &str) -> Vec<&str> {
    inp.split("\n")
        .filter(|line| line.len() > 0)
        .collect::<Vec<_>>()
}

#[derive(Debug)]
pub struct RuleEntry {
    count: usize,
    color: String,
}

pub struct RuleParser {
    regex: Regex,
}

impl RuleParser {
    pub fn new() -> Self {
        let regex = Regex::new("([0-9]+) ([a-z ]+)").unwrap();
        RuleParser { regex: regex }
    }

    // Goesn't :-(
    // pub fn parse(&self, rule: &str) -> (&str, Vec<RuleEntry>) {
        // (rule, vec![RuleEntry { count: 0, color: "foo".to_owned() }])
    // }

    pub fn parse(&self, rule: String) -> (String, Vec<RuleEntry>) {
        let left_right: Vec<_> = rule.split(" bags contain ").collect();
        // TODO: Is there a nicer way to do this?
        let left_color = left_right[0];
        let right_str = left_right[1];

        if right_str == "no other bags." {
            return (left_color.to_owned(), vec![])
        }

        let num_and_color_to_rule_entry = |string: &str| -> RuleEntry {
            let capture = self.regex.captures_iter(string).next().unwrap();
            let num = match capture[1].parse::<usize>() {
                Ok(num) => num,
                _ => panic!("Cannot parse as number: {}", &capture[1])
            };
            let color = &capture[2];

            RuleEntry { count: num, color: color.to_string() }
        };

        let rule_entries: Vec<_> = right_str
            .trim_end_matches(".")
            .split(", ")
            .map(|x| x.trim_end_matches(" bag").trim_end_matches(" bags"))
            .map(|x| num_and_color_to_rule_entry(x) )
            .collect();

        (left_color.to_owned(), rule_entries)
    }
}
