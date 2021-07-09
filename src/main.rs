use chrono::{DateTime, Datelike, Month, NaiveDateTime, Timelike, Utc};
use chrono_humanize::HumanTime;
use clap::{crate_authors, crate_description, crate_name, App, Arg};
use colored::Colorize;
use num_traits::FromPrimitive;

fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("timestamp")
                .takes_value(true)
                .help("A UNIX timestamp")
                .required(false),
        )
        .get_matches();

    // Get data
    let timestamp: i64 = matches
        .value_of("timestamp")
        .unwrap_or(Utc::now().timestamp().to_string().as_str())
        .parse()
        .expect("Not a valid timestamp");

    // Convert to a chrono format
    let date = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);

    // Convert to human-readable formats
    let human_readable = format!("{}", HumanTime::from(date));

    // Print the results
    println!(
        "{} is {} {} {}, at {} UTC {}",
        timestamp.to_string().cyan(),
        Month::from_u32(date.month()).unwrap().name().green(),
        date.day().to_string().green(),
        date.year().to_string().green(),
        format!(
            "{:02}:{:02}:{:02}",
            date.hour(),
            date.minute(),
            date.second(),
        )
        .green(),
        format!("({})", human_readable).bright_black()
    );
}
