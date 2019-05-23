// Description
// No more hiding from your alarm clock! You've decided you want your computer to keep you updated on the time so you're never late again. A talking clock takes a 24-hour time and translates it into words.
//
// Input Description
// An hour (0-23) followed by a colon followed by the minute (0-59).
//
// Output Description
// The time in words, using 12-hour format followed by am or pm.
//
// Sample Input data
// 00:00
// 01:30
// 12:05
// 14:01
// 20:29
// 21:00

// Sample Output data
// It's twelve am
// It's one thirty am
// It's twelve oh five pm
// It's two oh one pm
// It's eight twenty nine pm
// It's nine pm

use easy_shortcuts::traits::*;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a time (format hh:mm) as the first argument.");
        process::exit(0)
    }

    println!("{}", say(&args[1]))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn valid_values() {
        assert_eq!(say(&"00:00".to_string()), "It's twelve am".to_string());
        assert_eq!(say(&"01:30".to_string()), "It's one thirty am".to_string());
        assert_eq!(
            say(&"12:05".to_string()),
            "It's twelve oh five pm".to_string()
        );
        assert_eq!(say(&"14:01".to_string()), "It's two oh one pm".to_string());
        assert_eq!(
            say(&"20:29".to_string()),
            "It's eight twenty nine pm".to_string()
        );
        assert_eq!(say(&"21:00".to_string()), "It's nine pm".to_string())
    }
}

fn say(time: &String) -> String {
    let time_values: Vec<u32> = time
        .split(':')
        .map(|num_string| num_string.parse::<u32>().unwrap())
        .collect();

    let (hour, min_prefix, min, time_period) = match time_values.as_slice() {
        // Change 24h format into 12h format
        [h, m] if *h > 12 && *m > 0 && *m < 10 => (h - 12, "oh", *m, "pm"),
        [h, m] if *h > 12 => (*h - 12, "", *m, "pm"),
        [h, m] if *h == 12 && *m > 0 && *m < 10 => (*h, "oh", *m, "pm"),
        [h, m] if *h == 12 => (*h, "", *m, "pm"),
        // handle sub 10 minutes prefix
        [h, m] if *h == 0 && *m > 0 && *m < 10 => (12, "oh", *m, "am"),
        [h, m] if *m > 0 && *m < 10 => (*h, "oh", *m, "am"),
        // simple variations
        [h, m] if *h == 0 => (12, "", *m, "am"),
        [h, m] => (*h, "", *m, "am"),
        _ => panic!("Invalid time value!"),
    };

    format!(
        "It's {} {} {} {}",
        num_to_s(hour),
        min_prefix,
        num_to_s(min),
        time_period,
    )
    .split_whitespace()
    .join(' ')
}

// Format a number between 1 and 59 (inclusive) to a verbal format
fn num_to_s(num: u32) -> String {
    let singles = |n| match n {
        0 => "".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        _ => "_invalid_".to_string(),
    };

    let tens = |n| match n {
        2 => "twenty".to_string(),
        3 => "thirty".to_string(),
        4 => "forty".to_string(),
        5 => "fifty".to_string(),
        _ => "_invalid_".to_string(),
    };

    if num < 20 {
        singles(num)
    } else {
        match (num / 10, num % 10) {
            (2, 0) => tens(2),
            (2, x) => format!("{} {}", tens(2), singles(x)),
            (3, 0) => tens(3),
            (3, x) => format!("{} {}", tens(3), singles(x)),
            (4, 0) => tens(4),
            (4, x) => format!("{} {}", tens(4), singles(x)),
            (5, 0) => tens(5),
            (5, x) => format!("{} {}", tens(5), singles(x)),
            _ => "_invalid_".to_string(),
        }
    }
}
