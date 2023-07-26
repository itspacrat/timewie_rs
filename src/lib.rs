/*
historical: first scaffold was 7/24/2023 ~ 10:30a @itspacrat
*/
use std::error::Error;

#[allow(unused_imports)]
use std::time::{Duration,UNIX_EPOCH,SystemTime};

pub fn unix_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

/// a [u64] representing an hour in seconds
pub const UNIX_HOUR: u64 = 3600;
/// a [u64] representing the number of seconds in a 24-hour day
pub const UNIX_DAY: u64 = 86400;
/// a [u64] representing the number of seconds in a Unix Time year
pub const UNIX_YEAR: u64 = 31536000;

// expose a rough weekday api
#[repr(u8)]
/// intended use of `WeekDay::*` is for math regarding relative days of the week.
/// 
/// common use case: you have the start of the current week in Unix Time, 
pub enum WeekDay {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6
}

/// accepts a fuzzy string and parses a u64 Unix timestamp based on its information (or throws an [Error](std::error::Error)).
/// ### example:
/// ```rust
///     use timewie_rs::parse_fuzzy;
///     // assuming today's date is Jan 1st, 2023
///     // human-readable time string
///     let my_timer = "tomorrow 8:27pm";
///     let unix_timer = parse_fuzzy(my_timer)?;
///     
///     // the math here dictates this prints:
///     // time since unix epoch to 1/1/2023 +
///     // 1 day (in seconds) +
///     // 20 hours (8 pm in seconds on a 24 hour clock) +
///     // 27 minutes (in seconds)
///     //
///     // =  1672633620
///     println!("{}",unix_timer)
/// ```
pub fn parse_fuzzy<'a>(fuzzy_string: &'a str) -> Result<u64, Box<dyn Error>> {
    
    // the unix time thingy we're spitting out
    let stamp: u64;

    // split up the input by space and collect into a itertable type
    let args: Vec<_> = fuzzy_string.split(" ").collect::<Vec<&'a str>>();

    // TODO march for slashes (01/01/2023)
    // TODO march for a(m) p(m)
    // TODO march for colon / double colon (hh:mm / hh:mm:ss)
    // TODO march for relatives (today, yesterday, last week, next year)

    let mut timestamp_r: Vec<&str> = Vec::new();
    let mut date_r: Vec<&str> = Vec::new();
    let mut relative_r: Vec<&str> = Vec::new();

    for (i,a) in args.iter().enumerate() {

        let a = *a;

        // probably a date
        if
            a.contains("/") {
            date_r.push(a);
        }
        // probably a timestamp
        else if a.contains(":") {
            timestamp_r.push(a);
        }
        // filter common trash words
        else if
        a.contains("at") |
        a.contains("on") |
        a.contains("by") |
        a.contains("before") |
        a.contains("after") |
        a.contains("like") |
        a.contains("around") |
        a.contains("probably") |
        a.contains("about")
        {
            // skip them sumbitches
        }
        // probably a relative
        else {
            relative_r.push(a)
        }
        
    }

    println!("timestamp {:?}\ndate {:?}\nrelative {:?}\n",timestamp_r,date_r,relative_r);
    // `u_` prefix is meant to mean "unsigned int (unix time converted)"
    // just a fun little fact
    let mut u_date: u64 = 0;
    let mut u_time: u64 = 0;
    let mut u_relative: u64 = 0;

    for e in date_r {
        u_date = u_date + parse_date(e);
    }

    for e in timestamp_r {
        u_time = u_time + parse_timestamp(e);
    }
    for e in relative_r {
        u_relative = u_relative + parse_relative(e);
    }
    Ok(
        u_date+u_time+u_relative
    )
}
/// parses a timestamp (i.e. "1/1/2023", "01/01")
/// ### example:
/// ```rust
/// ```
pub fn parse_date<'s>(r_date: &'s str) -> u64 {
    // TODO implement
    0
}
/// parses a timestamp (i.e. "7" (24 hour clock by default = 7am), "6:13p" (6:13 in the evening), "2:12:03" (2:12 and 3 seconds in the morning))
/// ### example:
/// ```rust
/// ```
pub fn parse_timestamp<'s>(r_time: &'s str) -> u64 {
    // TODO implement
    0
}

/// parses a "relative" time (i.e. "tomorrow", "last week", "thursday", "in two years")
/// ### example:
/// ```rust
/// ```
pub fn parse_relative<'s>(r_rel: &'s str)-> u64 {
    let now = unix_now();
    let rel_out: u64;

    match r_rel.parse::<u64>() {
        // if it can't be parsed as a number, parse the words
        Err(_) => {
            rel_out = match r_rel {
                "yesterday" => {now - 86400}
                "today" => {now}
                "tomorrow" => {now + 86400}
                _ => {0}
            };
        }
        // if it can be parsed as a number, do that...?
        Ok(t) => {
            rel_out = UNIX_HOUR * t;
        }

    }
    
    rel_out
}
