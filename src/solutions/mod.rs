pub use super::Day;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
/*mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25; */

pub fn solve(day: Day, part: u8, input: &String) -> String {
    match day {
        1 => day1::solve(part, input),
        2 => day2::solve(part, input),
        3 => day3::solve(part, input),
        4 => day4::solve(part, input),
        5 => day5::solve(part, input),
        6 => day6::solve(part, input),
        7 => day7::solve(part, input),
        8 => day8::solve(part, input),
        9 => day9::solve(part, input),
        10 => day10::solve(part, input),
        11 => day11::solve(part, input),
        /*12 => day12::solve(part, input),
        13 => day13::solve(part, input),
        14 => day14::solve(part, input),
        15 => day15::solve(part, input),
        16 => day16::solve(part, input),
        17 => day17::solve(part, input),
        18 => day18::solve(part, input),
        19 => day19::solve(part, input),
        20 => day20::solve(part, input),
        21 => day21::solve(part, input),
        22 => day22::solve(part, input),
        23 => day23::solve(part, input),
        24 => day24::solve(part, input),
        25 => day25::solve(part, input), */
        _ => String::from(""),
    }
}
