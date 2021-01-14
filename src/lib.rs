pub mod db_manager;

use chrono::{DateTime, Local, NaiveDateTime, NaiveDate};
use rusqlite::{Result};
use rusqlite::NO_PARAMS;
use clap::ArgMatches;
use db_manager::{DbManager, LogRecord};

pub struct App {
    db_manager: DbManager
}

impl App {
    pub fn new() -> Result<App> {
        let db_manager = DbManager::new()?;
        Ok(App {
            db_manager
        })
    }

    pub fn run(&self, matches: ArgMatches) -> Result<()> {
        if let Some(ref matches) = matches.subcommand_matches("log") {
            let message = matches.value_of("message").unwrap();

            let time = match matches.value_of("time") {
                Some(custom_time) => NaiveDateTime::parse_from_str(custom_time, "%d.%m.%Y %H:%M").unwrap(),
                _ => Local::now().naive_local()
            };

            self.db_manager.insert(message, time);
        }

        if let Some(ref matches) = matches.subcommand_matches("ls") {
            let mut date = match matches.value_of("date") {
                Some(custom_date) => NaiveDate::parse_from_str(custom_date, "%d.%m.%Y").unwrap(),
                _ => Local::now().naive_local().date()
            };

            if matches.is_present("last") {
                date = self.db_manager.get_last_date().unwrap().date();
            }

            for row in self.db_manager.list_date(date).unwrap() {
                println!("{} {}", row.time, row.message)
            }
        }

        Ok(())
    }
}
