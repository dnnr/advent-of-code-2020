// Source: https://repl.it/@Scoder12/aoc-rust-template

// Days
pub mod day03;
pub mod day04;

pub fn noop(_inp: String) {}

pub type DayFn = fn(String);

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        3 => (day03::part1, day03::part2),
        4 => (day04::part1, noop),
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}
