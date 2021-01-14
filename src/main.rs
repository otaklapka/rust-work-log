use worklog::{App};
use std::error::Error;
use chrono::{DateTime, Local, NaiveDateTime, NaiveDate};
use rusqlite::{Result};
use clap::{App as Clap, Subcommand, Arg};
use chrono::format::Fixed::TimezoneOffset;

fn main() -> Result<()> {
    // worklog log "message" [-t="12.01.2021 14:05"]
    // worklog delete <time>
    // worklog set <time> "new message"
    // worklog ls [-d=12.01.2021, -l]

    let matches = Clap::new("Worklog")
        .version("1.0")
        .author("Ota Klapka <klapka.ota@gmail.com>")
        .subcommand(Clap::new("log")
            .arg(Arg::new("message")
                .required(true)
                .index(1)
            )
            .arg(Arg::new("time")
                .short('t')
                .takes_value(true)
            )
        )
        .subcommand(Clap::new("ls")
            .arg(Arg::new("date")
                .short('d')
                .takes_value(true)
            )
            .arg(Arg::new("last")
                .short('l')
            )
        )
        .subcommand(Clap::new("set")
            .arg(Arg::new("id")
                .required(true)
                .index(1)
            )
            .arg(Arg::new("new-message")
                .index(2)
            )
            .arg(Arg::new("time")
                .short('t')
                .takes_value(true)
            )
        )
        .subcommand(Clap::new("delete")
            .arg(Arg::new("id")
                .required(true)
                .index(1)
            )
        ).get_matches();

    let app = App::new()?;
    app.run(matches)?;

    Ok(())
}
