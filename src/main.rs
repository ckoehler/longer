use chrono::Duration;
use chrono::NaiveDate;
use clap::Parser;
use prettytable::{Table, row};

#[derive(Parser, Debug)]
#[clap(
    version,
    author = "Christoph Koehler",
    about = "Calculate when you've done something longer than not",
    long_about = "Have you ever wondered when you will have been married longer than not? This tool calculates that date for you!",
    after_help = "EXAMPLES:\n  longer --start 1990-05-15 --event 2015-08-20\n  longer -s 1985-01-01 -e 2010-06-15\n\nDATE FORMAT:\n  All dates must be in YYYY-MM-DD format (ISO 8601)"
)]
struct Opts {
    /// The start date, e.g. your birthday
    ///
    /// Format: YYYY-MM-DD (e.g., 1990-05-15)
    #[arg(short, long, value_name = "YYYY-MM-DD")]
    start: String,
    /// The event to count as the midpoint, e.g. your wedding date
    ///
    /// Format: YYYY-MM-DD (e.g., 2015-08-20)
    #[arg(short, long, value_name = "YYYY-MM-DD")]
    event: String,
}

fn main() {
    // parse command line options
    let opts: Opts = Opts::parse();

    // parse dates using chrono's built-in parsing
    let dt = NaiveDate::parse_from_str(&opts.start, "%Y-%m-%d")
        .expect("Invalid start date format (use yyyy-mm-dd)");
    let mid = NaiveDate::parse_from_str(&opts.event, "%Y-%m-%d")
        .expect("Invalid event date format (use yyyy-mm-dd)");

    // validate that event date comes after start date
    if mid <= dt {
        eprintln!("Error: Event date must be after start date");
        std::process::exit(1);
    }

    // grab the duration between Start and Event
    let diff = mid - dt;

    // the "Longer" date will now be Event + diff + 1 day, the first day when you will have been
    // ____ longer than not.
    let new = mid + diff + Duration::days(1);

    // display the results in a table
    let mut table = Table::new();
    table.add_row(row!["Start date", dt]);
    table.add_row(row!["Event date", mid]);
    table.add_row(row!["Days between", diff.num_days()]);
    table.add_row(row!["Longer date", new]);

    table.print_tty(false).unwrap();
}
