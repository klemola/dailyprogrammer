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
use std::error::Error;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a time (format hh:mm) as the first argument.");
        process::exit(0)
    }

    match say(&args[1]) {
        Ok(time_string) => println!("{}", time_string),
        Err(message) => println!("{}", message),
    }
}

fn say(time: &str) -> Result<String, Box<Error>> {
    let (h, m) = parse_time(&time)?;

    let (hour, min, time_period) = if h > 12 {
        (h - 12, m, "pm")
    } else if h == 0 {
        (12, m, "am")
    } else if h == 12 {
        (h, m, "pm")
    } else {
        (h, m, "am")
    };

    let min_prefix = if min > 0 && min < 10 { "oh" } else { "" };

    Ok(format!(
        "It's {} {} {} {}",
        num_to_s(hour),
        min_prefix,
        num_to_s(min),
        time_period,
    )
    .split_whitespace()
    .join(' '))
}

// Accepts a string in format "hh:mm" where hh and mm are valid values for 24h format
fn parse_time(time: &str) -> Result<(u32, u32), Box<Error>> {
    let time_values: Vec<&str> = time.split(':').to_vec();
    let err = Err(Box::from("Invalid time value"));

    match time_values.len() {
        2 => {
            let (hour_string, min_string) = (time_values[0], time_values[1]);
            let h: u32 = hour_string.parse::<u32>()?;
            let m: u32 = min_string.parse::<u32>()?;

            if h > 23 || m > 59 {
                err
            } else {
                Ok((h, m))
            }
        }
        _ => err,
    }
}

// Format a number between 1 and 59 (inclusive) to a verbal format
fn num_to_s(num: u32) -> String {
    let singles = |n| {
        String::from(match n {
            0 => "",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            _ => "_invalid_",
        })
    };

    let tens = |n| {
        String::from(match n {
            2 => "twenty",
            3 => "thirty",
            4 => "forty",
            5 => "fifty",
            _ => "_invalid_",
        })
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
            _ => String::from("_invalid_"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn valid_value_1() -> Result<(), Box<Error>> {
        let time_string = say(&String::from("00:00"))?;
        Ok(assert_eq!(time_string, String::from("It's twelve am")))
    }

    #[test]
    fn valid_value_2() -> Result<(), Box<Error>> {
        let time_string = say(&String::from("01:30"))?;
        Ok(assert_eq!(time_string, String::from("It's one thirty am")))
    }

    #[test]
    fn valid_value_3() -> Result<(), Box<Error>> {
        let time_string = say(&String::from("12:05"))?;
        Ok(assert_eq!(
            time_string,
            String::from("It's twelve oh five pm")
        ))
    }

    #[test]
    fn valid_value_4() -> Result<(), Box<Error>> {
        let time_string = say(&String::from("14:01"))?;
        Ok(assert_eq!(time_string, String::from("It's two oh one pm")))
    }

    #[test]
    fn valid_value_5() -> Result<(), Box<Error>> {
        let time_string = say(&String::from("20:29"))?;
        Ok(assert_eq!(
            time_string,
            String::from("It's eight twenty nine pm")
        ))
    }

    #[test]
    fn invalid_input() {
        assert!(say(&String::from("foobar")).is_err());
        assert!(say(&String::from("43:22")).is_err());
    }
}
