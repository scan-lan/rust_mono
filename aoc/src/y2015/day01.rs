use crate::Solution;

fn part_1(input: &str) -> Solution {
    let t = std::time::Instant::now();

    let output = input.chars().fold(0, |mut floor, el| match el {
        '(' => {
            floor += 1;
            floor
        }
        ')' => {
            floor -= 1;
            floor
        }
        _ => floor,
    });

    let output = output.to_string();
    let time_taken = t.elapsed();
    Solution { output, time_taken }
}

fn part_2(input: &str) -> Solution {
    let t = std::time::Instant::now();

    let output: i32 = input.chars().fold(0, |mut floor, el| match el {
        '(' => {
            floor += 1;
            floor
        }
        ')' => {
            floor -= 1;
            floor
        }
        _ => floor,
    });

    let output = output.to_string();
    let time_taken = t.elapsed();
    Solution { output, time_taken }
}

create_get_day!("./day01.json");
