static INPUT: &'static str = include_str!("./day01.txt");

pub fn part_1() -> i32 {
    INPUT.chars().fold(0, |mut floor, el| match el {
        '(' => {
            floor += 1;
            floor
        }
        ')' => {
            floor -= 1;
            floor
        }
        _ => floor,
    })
}

pub fn part_2() -> i32 {
    INPUT.chars().enumerate().fold(0, |mut floor, (i, el)| {
        if floor == -1 {
            println!("{}", i)
        }
        match el {
            '(' => {
                floor += 1;
                floor
            }
            ')' => {
                floor -= 1;
                floor
            }
            _ => floor,
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part_1() {
        println!("Day 1; pt 1: {}", part_1());
    }

    #[test]
    fn test_part_2() {
        println!("Day 1; pt 2: {}", part_2());
    }
}
