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

#[cfg(test)]
mod tests {
    use crate::{fizzbuzz, fizzbuzz_match};

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(String::from("1"), fizzbuzz(1));
        assert_eq!(String::from("fizz"), fizzbuzz(3));
        assert_eq!(String::from("buzz"), fizzbuzz(5));
        assert_eq!(String::from("fizzbuzz"), fizzbuzz(15))
    }

    #[test]
    fn test_fizzbuzz_match() {
        assert_eq!(String::from("1"), fizzbuzz_match(1));
        assert_eq!(String::from("fizz"), fizzbuzz_match(3));
        assert_eq!(String::from("buzz"), fizzbuzz_match(5));
        assert_eq!(String::from("fizzbuzz"), fizzbuzz_match(15))
    }
}
