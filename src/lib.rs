// Source: https://repl.it/@Scoder12/aoc-rust-template

#[macro_use]
extern crate lazy_static;

// Days
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
// pub mod day08;
pub mod day09;
// pub mod day10;
// pub mod day11;
pub mod day12;
pub mod day14;

pub fn noop(_inp: String) {}

pub type DayFn = fn(String);

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        // day01 driver: (absent)
        // day02 driver: yogan
        3 => (day03::part1, day03::part2), // driver: dnnr
        4 => (day04::part1, noop),         // driver: dnnr
        5 => (day05::part1, day05::part2), // driver: yogan
        6 => (day06::part1, day06::part2), // driver: dnnr
        7 => (day07::part1, noop),         // driver: dnnr
        // 8 => (day08::part1, day08::part2), // driver: yogan
        9 => (day09::part1, day09::part2), // driver: dnnr
        // 10 skipped
        // 11 skipped
        12 => (day12::part1, day12::part2), // driver: dnnr
        14 => (day14::part1, day14::part2), // driver: dnnr
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}
