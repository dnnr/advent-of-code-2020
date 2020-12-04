use std::collections::HashSet;
use std::iter::FromIterator;

pub fn part1(inp: String) {
    let passports = parse_input(&inp);

    let valid_count = passports.iter().filter(|x| is_valid(x)).count();
    println!("Valid: {}", valid_count);
}

pub fn part2(_: String) {}

fn parse_input(inp: &str) -> Vec<Vec<String>> {
    inp.split("\n\n")
        .filter(|line| line.len() > 0)
        .map(|line| line.replace("\n", " "))
        .map(|line| {
            line.split(" ")
                .map(|word| word.split(":").next().unwrap())
                .map(|word| word.to_owned())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
}

fn is_valid(passport: &Vec<String>) -> bool {
    let valid_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let fields: HashSet<String> = HashSet::from_iter(passport.iter().cloned());

    for valid_field in valid_fields {
        if !fields.contains(valid_field) {
            return false;
        }
    }

    true
}
