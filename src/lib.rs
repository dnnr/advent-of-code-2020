// Source: https://repl.it/@Scoder12/aoc-rust-template

// Days
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

pub fn noop(_inp: String) {}

pub type DayFn = fn(String);

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        // day01 driver: (absent)
        // day02 driver: yogan
        3 => (day03::part1, day03::part2),  // driver: dnnr
        4 => (day04::part1, noop),          // driver: dnnr
        5 => (day05::part1, day05::part2),  // driver: yogan
        6 => (day06::part1, day06::part2),  // driver: dnnr
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}
