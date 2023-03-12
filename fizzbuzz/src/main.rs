fn fizzbuzz(n: u32) -> String {
    let mut answer = String::new();

    if n % 3 == 0 {
        answer.push_str("fizz");
    }
    if n % 5 == 0 {
        answer.push_str("buzz");
    }
    if answer.len() == 0 {
        answer.push_str(&n.to_string());
    }

    return answer;
}

fn fizzbuzz_match(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => String::from("fizzbuzz"),
        (0, _) => String::from("fizz"),
        (_, 0) => String::from("buzz"),
        _ => n.to_string(),
    }
}

fn main() {
    for n in 1..=15 {
        println!("{}", fizzbuzz_match(n))
    }
    for n in 1..=15 {
        println!("{}", fizzbuzz(n))
    }
}

// 1 2 fizz 4 buzz fizz 7 8 fizz buzz 11 fizz 13 14 fizzbuzz
