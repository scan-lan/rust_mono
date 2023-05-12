use itertools::Itertools;

use std::sync::Once;

static INPUT: &'static str = include_str!("./day02.txt");
static INIT: Once = Once::new();
static mut BOXES: Vec<GiftBox> = Vec::new();

/// Get a static reference to list of gift boxes
fn input_gift_boxes() -> &'static Vec<GiftBox> {
    unsafe {
        INIT.call_once(|| {
            BOXES.extend(
                INPUT
                    .lines()
                    .map(|line| {
                        let dimensions = line
                            .split('x')
                            .map(|d| d.parse::<u64>().expect("Invalid data entered"))
                            .collect_vec();

                        GiftBox::from(dimensions)
                    })
                    .collect_vec(),
            )
        });
        &BOXES
    }
}

impl From<Vec<u64>> for GiftBox {
    fn from(value: Vec<u64>) -> Self {
        let mut value = value.iter();

        let (l, w, h) = (
            *value.next().expect("Invalid data entered"),
            *value.next().expect("Invalid data entered"),
            *value.next().expect("Invalid data entered"),
        );

        let combos = [l, w, h].into_iter().combinations(2);

        let surface_area = combos
            .clone()
            .map(|c| c.into_iter().product::<u64>() * 2)
            .sum();

        let slack = combos
            .clone()
            .map(|c| c.into_iter().product())
            .min()
            .expect("Should be a min");

        let bow = [l, w, h].into_iter().product::<u64>();

        let ribbon_required = combos
            .clone()
            // Map to perimeters
            .map(|c| c.into_iter().sum::<u64>() * 2)
            .min()
            .expect("Min perimeter")
            + bow;

        GiftBox {
            l,
            w,
            h,
            surface_area,
            slack,
            ribbon_required,
        }
    }
}

struct GiftBox {
    l: u64,
    w: u64,
    h: u64,
    surface_area: u64,
    slack: u64,
    ribbon_required: u64,
}

// impl GiftBox {
//     fn surface_area(&self) -> u64 {
//         self.surface_area
//     }
//
//     fn slack(&self) -> u64 {
//         self.slack
//     }
//
//     fn ribbon_required(&self) -> u64 {
//         self.slack
//     }
// }

pub fn part_1() -> u64 {
    let boxes = input_gift_boxes();

    boxes.iter().map(|bx| bx.surface_area + bx.slack).sum()
}

pub fn part_2() -> u64 {
    let boxes = input_gift_boxes();

    boxes.iter().map(|bx| bx.ribbon_required).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part_1() {
        println!("Day 2; pt 1: {}", part_1());
    }

    #[test]
    fn test_part_2() {
        println!("Day 2; pt 2: {}", part_2());
    }
}
