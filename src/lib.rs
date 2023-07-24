/*
historical: first scaffold was 7/24/2023 ~ 10:30a @itspacrat
*/
use std::error::Error;

#[allow(unused_imports)]
use std::time::{Duration,UNIX_EPOCH,SystemTime};

/// accepts a fuzzy string and parses a u64 Unix timestamp based on its information (or throws an error).
/// example:
/// ```rust
///     // assuming today's date is Jan 1st, 2023
///     // human readable-time string
///     let my_timer = "tomorrow 8:27pm";
///     let unix_timer = parse_time(my_timer)?;
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

    let mut r_date: u64 = 0;
    let mut r_time: u64 = 0;
    let mut r_relative: u64 = 0;
    
    
    Ok(
        r_date+r_time+r_relative
    )
}

pub fn parse_date() {

}
pub fn parse_time() {
    
}
pub fn parse_relative() {
    
}
