use chrono::Duration;
use chrono::NaiveDate;
use clap::Parser;
use prettytable::{row, Table};

/// Tool to handle discussion guides
#[derive(Parser, Debug)]
#[clap(version, author = "Christoph Koehler")]
struct Opts {
    /// The Start date, e.g. your birthday
    #[arg(short, long)]
    start: String,
    /// The event to count as the midpoint
    #[arg(short, long)]
    event: String,
}

fn main() {
    // parse command line options
    let opts: Opts = Opts::parse();

    // parse start date into year, month, day Vec
    let start: Vec<u32> = opts
        .start
        .split('-')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    // parse event date into year, month, day Vec
    let mid: Vec<u32> = opts
        .event
        .split('-')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    // now parse those into NaiveDates
    let dt = NaiveDate::from_ymd_opt(start[0].try_into().unwrap(), start[1], start[2]).unwrap();
    let mid = NaiveDate::from_ymd_opt(mid[0].try_into().unwrap(), mid[1], mid[2]).unwrap();

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
