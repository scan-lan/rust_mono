use itertools::Itertools;

static INPUT: &'static str = include_str!("./day02.txt");

struct GiftBox {
    l: u32,
    w: u32,
    h: u32,
}

impl From<Vec<u32>> for GiftBox {
    fn from(value: Vec<u32>) -> Self {
        let mut value = value.iter();

        GiftBox {
            l: *value.next().expect("Invalid data entered"),
            w: *value.next().expect("Invalid data entered"),
            h: *value.next().expect("Invalid data entered"),
        }
    }
}

impl GiftBox {
    fn surface_area(&self) -> u32 {
        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l
    }

    fn min_area(&self) -> u32 {
        [self.l, self.w, self.h]
            .iter()
            .combinations(2)
            .map(|c| c.into_iter().product())
            .min()
            .expect("Should be a min")
    }
}

pub fn part_1() -> u32 {
    INPUT.lines().fold(0, |mut tot, el| {
        let dimensions: Vec<u32> = el
            .split('x')
            .map(|d| d.parse::<u32>().unwrap_or(1))
            .collect();
        let areas = dimensions
            .into_iter()
            .combinations(2)
            .map(|side| side.into_iter().product());
        let slack = areas.clone().min().expect("There will be a min");
        let box_area = areas.fold(0, |mut acc, area: u32| {
            acc += 2 * area;
            acc
        });

        tot += box_area + slack;
        tot
    })
}

// pub fn part_2() -> i32 {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part_1() {
        println!("Day 2; pt 1: {}", part_1());
    }

    // #[test]
    // fn test_part_2() {
    //     println!("Day 2; pt 2: {}", part_2());
    // }
}
