use chrono::Duration;
use chrono::NaiveDate;
use clap::Parser;
// use thiserror::Error;
// use tracing::*;
// use date_time_parser::DateParser;
use prettytable::{row, Table};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;

/// Tool to handle discussion guides
#[derive(Parser, Debug)]
#[clap(version, author = "Christoph Koehler")]
struct Opts {
    /// The Start date, e.g. your birthday
    #[arg(short, long)]
    start: String,
    /// The event to count as the midpoint
    #[arg(short, long)]
    midpoint: String,
}

fn main() {
    let format = fmt::format().with_target(false);

    // Create a `fmt` collector that uses our custom event format, and set it
    // as the default.
    // Also look for RUST_LOG var, but default to INFO level if not set.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .event_format(format)
        .init();

    // parse command line options
    let opts: Opts = Opts::parse();
    // parse start date
    let start: Vec<u32> = opts
        .start
        .split('-')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let mid: Vec<u32> = opts
        .midpoint
        .split('-')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let dt = NaiveDate::from_ymd_opt(start[0].try_into().unwrap(), start[1], start[2]).unwrap();
    let mid = NaiveDate::from_ymd_opt(mid[0].try_into().unwrap(), mid[1], mid[2]).unwrap();

    let diff = mid - dt;

    let new = mid + diff + Duration::days(1);
    let mut table = Table::new();
    table.add_row(row!["Start date", dt]);
    table.add_row(row!["Event date", mid]);
    table.add_row(row!["Days between", diff.num_days()]);
    table.add_row(row!["Longer date", new]);

    table.print_tty(false).unwrap();
}
