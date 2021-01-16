pub mod db_manager;

use chrono::{Local, NaiveDateTime, NaiveDate};
use clap::ArgMatches;
use db_manager::{DbManager};
use std::str::FromStr;
use std::process;
use prettytable::{Table, row, cell, format};

pub struct App {
    db_manager: DbManager
}

impl App {
    pub fn new(db_file_name: &str) -> App {
        let db_manager = DbManager::new(db_file_name).unwrap_or_else(|err| {
            println!("Failed to init database connection error: {:#?}", err);
            process::exit(1);
        });

        App {
            db_manager
        }
    }

    pub fn run(&self, matches: &ArgMatches) {
        match matches.subcommand() {
            Some(("log", log_matches)) => self.log(log_matches),
            Some(("ls", ls_matches)) => self.ls(ls_matches),
            Some(("set", set_matches)) => self.set(set_matches),
            Some(("delete", delete_matches)) => self.delete(delete_matches),
            _ => ()
        }
    }

    fn log(&self, matches: &ArgMatches) {
        let message = matches.value_of("message").unwrap();

        let time = match matches.value_of("time") {
            Some(custom_time) => NaiveDateTime::parse_from_str(custom_time, "%d.%m.%Y %H:%M").unwrap_or_else(|_err| {
                println!("Provided datetime is not in '10.01.2021 14:05' format");
                process::exit(1);
            }),
            _ => Local::now().naive_local()
        };

        match self.db_manager.insert(message, time) {
            Ok(_) => println!("{}  {}", time.format("%d.%m.%Y %H:%M"), message),
            Err(err) => println!("Insert error: {:#?}", err)
        }
    }

    fn ls(&self, matches: &ArgMatches) {
        let mut date = match matches.value_of("date") {
            Some(custom_date) => NaiveDate::parse_from_str(custom_date, "%d.%m.%Y").unwrap_or_else(|_err| {
                println!("Provided date is not in '10.01.2021' format");
                process::exit(1);
            }),
            _ => Local::now().naive_local().date()
        };

        if matches.is_present("last") {
            date = self.db_manager.get_last_date().unwrap_or(Local::now().naive_local()).date();
        }

        println!("Records from {}:\n", date.format("%d.%m.%Y"));

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_CLEAN);
        table.set_titles(row![u -> "ID", u -> "Time", u -> "Message"]);

        for row in self.db_manager.list_date(date).unwrap() {
            table.add_row(row![row.id, row.time.format("%H:%M"), row.message]);
        }

        table.printstd();
    }

    fn set(&self, matches: &ArgMatches) {
        let id: u32 = FromStr::from_str(matches.value_of("id").unwrap()).unwrap_or_else(|_err| {
            println!("Provided row ID is not a number");
            process::exit(1);
        });

        let new_message = matches.value_of("message");

        let date = match matches.value_of("time") {
            Some(custom_date) => Some(NaiveDateTime::parse_from_str(custom_date, "%d.%m.%Y %H:%M").unwrap_or_else(|_err| {
                println!("Provided datetime is not in '10.01.2021 14:05' format");
                process::exit(1);
            })),
            _ => None
        };

        match self.db_manager.set(id, new_message, date) {
            Ok(_) => println!("Record with ID {} was updated successfully", id),
            Err(err) => println!("Update error: {:#?}", err)
        }
    }

    fn delete(&self, matches: &ArgMatches) {
        let id: u32 = FromStr::from_str(matches.value_of("id").unwrap()).unwrap_or_else(|_err| {
            println!("Provided row ID is not a number");
            process::exit(1);
        });

        match self.db_manager.delete(id) {
            Ok(_) => println!("Record with ID {} was deleted successfully", id),
            Err(err) => println!("Delete error: {:#?}", err)
        }
    }
}
