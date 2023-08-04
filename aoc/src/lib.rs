use std::time::Duration;

use serde::{Deserialize, Serialize};

pub struct Year {
    pub year: u16,
    pub days: Vec<DaySolution>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayInfo {
    part_1: String,
    part_2: String,
    input: String,
}

pub struct Solution {
    pub output: String,
    pub time_taken: Duration,
}

pub struct DaySolution {
    pub info: DayInfo,
    pub parts: (fn(&str) -> Solution, fn(&str) -> Solution),
}

pub struct Options {
    pub year: u16,
    pub day: u8,
    pub part: Option<u8>,
}

pub fn show_solutions(options: Options) {
    let years = get_years();

    years.iter().for_each(|year| {
        year.days.iter().enumerate().for_each(|(n, day)| {
            println!("DAY {}\n===============\n\n", n + 1);
            (0..=1).for_each(|i| {
                let solution = (if i == 0 { day.parts.0 } else { day.parts.1 })(&day.info.input);
                println!(
                    "Part {}:\n\n{}\n\nSolution: {}\nTime taken: {:?}",
                    i + 1,
                    (if i == 0 {
                        &day.info.part_1
                    } else {
                        &day.info.part_2
                    }),
                    solution.output,
                    solution.time_taken
                );
            });
        })
    });
}

macro_rules! create_get_day {
    ($day_file:literal) => {
        pub fn get_day() -> crate::DaySolution {
            let day_json = include_str!($day_file);
            let day_info: crate::DayInfo =
                serde_json::from_str(day_json).expect("the file should be correctly formatted");

            crate::DaySolution {
                info: day_info,
                parts: (part_1, part_2),
            }
        }
    };
}

mod y2015;

macro_rules! get_year {
    ($year:ident, $years:ident) => {
        use $year::{
            day01,
            day02,
            // day03,
            // day04,
            // day05,
            // day06,
            // day07,
            // day08,
            // day09,
            // day10,
            // day11,
            // day12,
            // day13,
            // day14,
            // day15,
            // day16,
            // day17,
            // day18,
            // day19,
            // day20,
            // day21,
            // day22,
            // day23,
            // day24,
            // day25,
        };
        $years.push(Year {
            year: 2015,
            days: vec![
                day01::get_day(),
                day02::get_day(),
                // day03::get_day(),
                // day04::get_day(),
                // day05::get_day(),
                // day06::get_day(),
                // day07::get_day(),
                // day08::get_day(),
                // day09::get_day(),
                // day10::get_day(),
                // day11::get_day(),
                // day12::get_day(),
                // day13::get_day(),
                // day14::get_day(),
                // day15::get_day(),
                // day16::get_day(),
                // day17::get_day(),
                // day18::get_day(),
                // day19::get_day(),
                // day20::get_day(),
                // day21::get_day(),
                // day22::get_day(),
                // day23::get_day(),
                // day24::get_day(),
                // day25::get_day(),
            ],
        });
    };
}

pub fn get_years() -> Vec<Year> {
    let mut years = Vec::new();

    get_year!(y2015, years);

    years
}
