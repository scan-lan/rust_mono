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
    pub part_1: fn(&str) -> Solution,
    pub part_2: fn(&str) -> Solution,
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
            let solution_1 = (day.part_1)(&day.info.input);
            println!(
                "Part 1:\n\n{}\n\nSolution: {}\nTime taken: {:?}",
                day.info.part_1, solution_1.output, solution_1.time_taken
            );
            let solution_2 = (day.part_2)(&day.info.input);
            println!(
                "\n---------------\n\nPart 2:\n\n{}\n\nSolution: {}\nTime taken: {:?}\n",
                day.info.part_2, solution_2.output, solution_2.time_taken
            );
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
                part_1,
                part_2,
            }
        }
    };
}

mod y2015;
use y2015::{day01, day02};

pub fn get_years() -> Vec<Year> {
    let mut years = Vec::new();

    years.push(Year {
        year: 2015,
        days: vec![
            day01::get_day(),
            day02::get_day(),
            // ...
        ],
    });

    years
}
