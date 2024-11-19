use itertools::Itertools;

use std::sync::Once;

use crate::Solution;

static INIT: Once = Once::new();
static mut BOXES: Vec<GiftBox> = Vec::new();

/// Get a static reference to list of gift boxes
fn input_gift_boxes(input: &str) -> &'static Vec<GiftBox> {
    unsafe {
        INIT.call_once(|| {
            BOXES.extend(
                input
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
            // l,
            // w,
            // h,
            surface_area,
            slack,
            ribbon_required,
        }
    }
}

struct GiftBox {
    // l: u64,
    // w: u64,
    // h: u64,
    surface_area: u64,
    slack: u64,
    ribbon_required: u64,
}

pub fn part_1(input: &str) -> Solution {
    let boxes = input_gift_boxes(input);
    let t = std::time::Instant::now();

    let output: u64 = boxes.iter().map(|bx| bx.surface_area + bx.slack).sum();

    let output = output.to_string();
    let time_taken = t.elapsed();
    Solution { output, time_taken }
}

pub fn part_2(input: &str) -> Solution {
    let t = std::time::Instant::now();

    let boxes = input_gift_boxes(input);

    let output: u64 = boxes.iter().map(|bx| bx.ribbon_required).sum();

    let output = output.to_string();
    let time_taken = t.elapsed();
    Solution { output, time_taken }
}

create_get_day!("./day02.json");
